mod test;
mod action;
pub mod cache;
pub mod progress;
mod location;
pub mod scene;
pub use cache::Cache;
use location::{LocationIdentifier, Location};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::{HashMap,HashSet};
use progress::Progress;
use scene::{
    SceneIdentifier,
    Scene,
    SceneView,
    MenuItemIdentifier,
};

pub type Identifier = String;
pub type ObjectIdentifier = Identifier;
pub type CommodityIdentifier = Identifier;
pub type AchievementIdentifier = Identifier;

#[derive(Debug,Deserialize,Serialize,Default,Clone)]
pub struct Object {
    pub name: String,
    pub description: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Achievement {
    pub name: String,
    pub description: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Commodity {
    pub name: String,
    pub description: String,
}

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash,Default)]
pub enum State {
    #[default]
    Playing,
    Won,
    Lost,
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Character {
    #[serde(default)]
    pub stats: HashMap<String, f32>,
    #[serde(default)]
    pub inventory: Cache,
    #[serde(default)]
    pub achievements: HashSet<AchievementIdentifier>,
    #[serde(default)]
    pub state: State,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Game {
    #[serde(default)]
    pub locations: HashMap<LocationIdentifier, Location>,
    pub character: Character,
    pub scenes: HashMap<SceneIdentifier, Scene>,
    pub start_scene: SceneIdentifier,
}

impl Game {
    pub fn from_yaml(game_yaml: impl std::io::Read) -> Result<Game, Box<dyn Error>> {
        let game: Game = serde_yaml::from_reader(game_yaml)?;
        Ok(game)
    }

    fn menu_items(&self, character: &Character, scene: &Scene, menu: &mut HashMap<MenuItemIdentifier, String>) {
        menu.clear();
        for (id, item) in &scene.menu {
            log::debug!("Testing menu item {:?} {:?}", id, item.test);
            if match item.test.as_ref() {
                Some(test) => test.test(character),
                None => true,
            } {
                log::debug!("Menu item {:?} is available", id);
                menu.insert(id.clone(), item.description.clone());
            }
        }
    }

    pub fn initialize(&self, progress: &mut Progress, view: &mut SceneView) {
        progress.scene = self.start_scene.clone();
        progress.character = self.character.clone();
        let start_scene = self.scenes.get(&progress.scene).unwrap();
        view.description = start_scene.render_for(self, progress).unwrap();
        self.menu_items(&progress.character, start_scene, &mut view.menu);
    }

    pub fn choose(
        &self,
        progress: &mut Progress,
        choice: &MenuItemIdentifier,
        view: &mut SceneView) -> Result<bool, Box<dyn Error>> {

        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        let menu_item = scene.menu.get(choice).ok_or_else(|| format!("Menu item {:?} not found", choice))?;
        for action in &menu_item.actions {
            log::debug!("Action: {:?}", action);
            if action == &action::Action::Quit {
                return Ok(false);
            }
            if !action.act(progress) {
                break
            }
        }
        if progress.character.state != State::Playing {
            return Ok(false);
        }
        log::debug!("Updated Character: {:?}", progress.character);
        progress.scene = menu_item.next_scene.clone();
        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        view.description = self.scenes.get(&progress.scene).ok_or("Scene not found")?.render_for(self, progress)?;
        self.menu_items(&progress.character, scene, &mut view.menu);
        Ok(true)
    }
}

