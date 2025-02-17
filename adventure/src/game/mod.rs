mod test;
mod action;
use test::Test;
use action::Action;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::{HashMap,HashSet};
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Identifier(String);

impl Identifier {
    pub fn from_string(name: &str) -> Self {
        Identifier(name.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn empty() -> Self {
        Identifier("".to_string())
    }
}
impl PartialEq<String> for Identifier {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}
impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
impl PartialEq<Identifier> for Identifier {
    fn eq(&self, other: &Identifier) -> bool {
        self.0 == *other.0
    }
}
impl Eq for Identifier {}
impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
pub type ObjectIdentifier = Identifier;
pub type CommodityIdentifier = Identifier;
pub type AchievementIdentifier = Identifier;
pub type SceneIdentifier = Identifier;
pub type MenuItemIdentifier = Identifier;

#[derive(Debug,Deserialize,Serialize)]
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
    pub inventory: HashSet<ObjectIdentifier>,
    #[serde(default)]
    pub achievements: HashSet<AchievementIdentifier>,
    #[serde(default)]
    pub commodities: HashMap<CommodityIdentifier, u32>,
    #[serde(default)]
    pub state: State,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct MenuItem {
    pub description: String,
    pub test: Option<Test>,
    #[serde(default)]
    pub action: Action,
    pub next_scene: SceneIdentifier,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Scene {
    pub description: String,
    pub menu: HashMap<MenuItemIdentifier, MenuItem>,
}

impl Scene {
    pub fn render_for(&self, character: &Character) -> tera::Result<String> {
        let mut renderer = tera::Tera::default();
        let context = tera::Context::from_serialize(character).unwrap();
        renderer.render_str(&self.description, &context)
    }
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Game {
    #[serde(default)]
    pub objects: HashMap<ObjectIdentifier, Object>,
    #[serde(default)]
    pub commodities: HashMap<CommodityIdentifier, Commodity>,
    #[serde(default)]
    pub achievements: HashMap<AchievementIdentifier, Achievement>,
    pub character: Character,
    pub scenes: HashMap<SceneIdentifier, Scene>,
    pub start_scene: SceneIdentifier,
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Progress {
    pub scene: SceneIdentifier,
    pub character: Character,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct SceneView {
    pub description: String,
    pub menu: HashMap<MenuItemIdentifier, String>,
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
        view.description = start_scene.render_for(&progress.character).unwrap();
        self.menu_items(&progress.character, start_scene, &mut view.menu);
    }

    pub fn choose(
        &self,
        progress: &mut Progress,
        choice: &MenuItemIdentifier,
        view: &mut SceneView) -> Result<bool, Box<dyn Error>> {

        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        let menu_item = scene.menu.get(choice).ok_or_else(|| format!("Menu item {:?} not found", choice))?;
        menu_item.action.act(&mut progress.character);
        if progress.character.state != State::Playing {
            return Ok(false);
        }
        log::debug!("Updated Character: {:?}", progress.character);
        progress.scene = menu_item.next_scene.clone();
        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        view.description = self.scenes.get(&progress.scene).ok_or("Scene not found")?.render_for(&progress.character)?;
        self.menu_items(&progress.character, scene, &mut view.menu);
        Ok(true)
    }
}

