use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::{HashMap,HashSet};

#[derive(Deserialize,Serialize)]
pub struct Test {
    pub stat_greater: Option<(String, f32)>,
    pub stat_less: Option<(String, f32)>,
    pub inventory_contains: Option<ObjectIdentifier>,
    pub achievement_earned: Option<AchievementIdentifier>,
    pub not: Option<Box<Test>>,
    pub and: Option<(Box<Test>, Box<Test>)>,
    pub or: Option<(Box<Test>, Box<Test>)>,
    pub any: Option<Vec<Test>>,
    pub all: Option<Vec<Test>>,
    pub none: Option<Vec<Test>>,
}

impl std::fmt::Debug for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Test { stat_greater: Some((stat, value)), .. } => write!(f, "{} > {}", stat, value),
            Test { stat_less: Some((stat, value)), .. } => write!(f, "{} < {}", stat, value),
            Test { inventory_contains: Some(object), .. } => write!(f, "inventory contains {:?}", object),
            Test { achievement_earned: Some(achievement), .. } => write!(f, "earned {:?}", achievement),
            Test { not: Some(test), .. } => write!(f, "not {:?}", test),
            Test { and: Some((test1, test2)), .. } => write!(f, "{:?} and {:?}", test1, test2),
            Test { or: Some((test1, test2)), .. } => write!(f, "{:?} or {:?}", test1, test2),
            Test { any: Some(tests), .. } => write!(f, "any of {:?} is true", tests),
            Test { all: Some(tests), .. } => write!(f, "all of {:?} are true", tests),
            Test { none: Some(tests), .. } => write!(f, "none of {:?} are true", tests),
            _ => write!(f, "no test defined"),
        }
    }
}

impl Test {
    pub fn test(&self, character: &Character) -> bool {
        match self {
            Test { stat_greater: Some((stat, value)), .. } => {
                let ret = character.stats.get(stat).is_some_and(|v| v > value);
                log::debug!("Testing {} > {} = {}", stat, value, ret);
                ret
            }
            Test { stat_less: Some((stat, value)), .. } => character.stats.get(stat).is_some_and(|v| v < value),
            Test { inventory_contains: Some(object), .. } => {
                let ret = character.inventory.contains(object);
                log::debug!("Testing inventory contains {:?} = {}", object, ret);
                ret
            }
            Test { achievement_earned: Some(achievement), .. } => character.achievements.contains(achievement),
            Test { not: Some(test), .. } => {
                let ret = !test.test(character);
                log::debug!("Testing not {:?} = {}", test, ret);
                ret
            }
            Test { and: Some((test1, test2)), .. } => test1.test(character) && test2.test(character),
            Test { or: Some((test1, test2)), .. } => test1.test(character) || test2.test(character),
            Test { any: Some(tests), .. } => tests.iter().any(|test| test.test(character)),
            Test { all: Some(tests), .. } => tests.iter().all(|test| test.test(character)),
            Test { none: Some(tests), .. } => tests.iter().all(|test| !test.test(character)),
            _ => false,
        }
    }
}

#[derive(Debug,Deserialize,Serialize,Default)]
pub enum Action {
    #[default]
    None,
    AddStat(String, f32),
    AddToInventory(ObjectIdentifier),
    AddAchievement(AchievementIdentifier),
    Win,
    Lose,
}

impl Action {
    pub fn act(&self, character: &mut Character) {
        log::debug!("Acting on character with {:?}", self);
        match self {
            Action::None => {}
            Action::AddStat(stat, value) => {
                let stat_value = character.stats.entry(stat.clone()).or_insert(0.0);
                *stat_value += value;
            }
            Action::AddToInventory(object) => {
                log::debug!("Adding object {:?}", object);
                character.inventory.insert(object.clone());
            }
            Action::AddAchievement(achievement) => {
                character.achievements.insert(achievement.clone());
            }
            Action::Win => {
                character.state = State::Won;
            }
            Action::Lose => {
                character.state = State::Lost;
            }
        }
    }
}

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

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash,Default)]
pub enum State {
    #[default]
    Playing,
    Won,
    Lost,
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Character {
    pub stats: HashMap<String, f32>,
    pub inventory: HashSet<ObjectIdentifier>,
    pub achievements: HashSet<AchievementIdentifier>,
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

#[derive(Debug,Deserialize,Serialize)]
pub struct Game {
    pub objects: HashMap<ObjectIdentifier, Object>,
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
            if item.test.as_ref().is_some_and(|test| test.test(character)) {
                log::debug!("Menu item {:?} is available", id);
                menu.insert(id.clone(), item.description.clone());
            }
        }
    }

    pub fn initialize(&self, progress: &mut Progress, view: &mut SceneView) {
        progress.scene = self.start_scene.clone();
        progress.character = self.character.clone();
        let start_scene = self.scenes.get(&progress.scene).unwrap();
        view.description = start_scene.description.clone();
        self.menu_items(&progress.character, start_scene, &mut view.menu);
    }

    pub fn choose(
        &self,
        progress: &mut Progress,
        choice: &MenuItemIdentifier,
        view: &mut SceneView) -> Result<bool, Box<dyn Error>> {

        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        let menu_item = scene.menu.get(choice).ok_or("Menu item not found")?;
        menu_item.action.act(&mut progress.character);
        if progress.character.state != State::Playing {
            return Ok(false);
        }
        log::debug!("Updated Character: {:?}", progress.character);
        progress.scene = menu_item.next_scene.clone();
        let scene = self.scenes.get(&progress.scene).ok_or("Scene not found")?;
        view.description = self.scenes.get(&progress.scene).ok_or("Scene not found")?.description.clone();
        self.menu_items(&progress.character, scene, &mut view.menu);
        Ok(true)
    }
}

