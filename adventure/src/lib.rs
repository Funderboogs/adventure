use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::{HashMap,HashSet};
use cursive::{
    views::{SelectView,TextView,LinearLayout,Button,ScrollView,Dialog},
    Cursive, CursiveExt,
};

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
            Test { stat_greater: Some((stat, value)), .. } => character.stats.get(stat).is_some_and(|v| v > value),
            Test { stat_less: Some((stat, value)), .. } => character.stats.get(stat).is_some_and(|v| v < value),
            Test { inventory_contains: Some(object), .. } => character.inventory.contains(object),
            Test { achievement_earned: Some(achievement), .. } => character.achievements.contains(achievement),
            Test { not: Some(test), .. } => !test.test(character),
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

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash)]
pub struct ObjectIdentifier(String);

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash)]
pub struct AchievementIdentifier(String);

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash)]
pub struct SceneIdentifier(String);

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq,Eq,Hash)]
pub struct MenuItemIdentifier(String);

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

pub trait Driver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>>;
}

impl Game {
    pub fn from_yaml(game_yaml: impl std::io::Read) -> Result<Game, Box<dyn Error>> {
        let game: Game = serde_yaml::from_reader(game_yaml)?;
        Ok(game)
    }

    fn menu_items(&self, character: &Character, scene: &Scene, menu: &mut HashMap<MenuItemIdentifier, String>) {
        for (id, item) in &scene.menu {
            log::debug!("Testing menu item {:?}", id);
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
        view.menu.clear();
        progress.scene = menu_item.next_scene.clone();
        view.description = self.scenes.get(&progress.scene).ok_or("Scene not found")?.description.clone();
        self.menu_items(&progress.character, scene, &mut view.menu);
        Ok(true)
    }
}

pub struct ConsoleDriver<'game> {
    pub progress: Progress,
    sui: Cursive,
    game: &'game Game,
}

impl<'game> ConsoleDriver<'game> {
    pub fn new(game: &'game Game) -> Self {
        ConsoleDriver {
            progress: Progress {
                scene: SceneIdentifier("".to_string()),
                character: Character {
                    stats: HashMap::new(),
                    inventory: HashSet::new(),
                    achievements: HashSet::new(),
                    state: State::Playing,
                },
            },
            sui: Cursive::default(),
            game,
        }
    }
    fn view(&mut self, view: &SceneView) {
        self.sui.clear();
        self.sui.add_layer(
            LinearLayout::vertical()
                .child(
                    ScrollView::new(
                        TextView::new(view.description.clone()),
                    ),
                )
                .child(
                    Button::new("OK", |s| {
                        s.quit();
                    }),
                ),
        );
        self.sui.run();
    }
    fn get_choice(&mut self, view: &SceneView) -> Option<MenuItemIdentifier> {
        let mut select = SelectView::new();
        for (id, description) in &view.menu {
            select.add_item(description, id.clone());
        }
        select.add_item("Quit", MenuItemIdentifier("__QUIT".to_string()));
        select.set_on_submit(|s, choice| {
            if *choice != MenuItemIdentifier("__QUIT".to_string()) {
                s.set_user_data(choice.clone());
            }
            s.quit();
        });
        self.sui.add_layer(select);
        self.sui.run();
        self.sui.user_data().cloned()
    }
    
}

impl<'game> Driver<'game> for ConsoleDriver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>> {
        let mut view = SceneView {
            description: String::new(),
            menu: HashMap::new(),
        };
        self.game.initialize(&mut self.progress, &mut view);
        self.sui = Cursive::default();
        loop {
            self.view(&view);
            if let Some(choice) = self.get_choice(&view) {
                if !self.game.choose(&mut self.progress, &choice, &mut view)? {
                  break
                }
            } else {
                break;
            }
        }
        let message = if self.progress.character.state == State::Won {
            "Congratulations! You won!"
        } else if self.progress.character.state == State::Lost {
            "Sorry, you lost."
        } else {
            "Meh, you left."
        }.to_string();
        self.sui.clear();
        self.sui.add_layer(
            Dialog::around(TextView::new(message))
                .button("OK", |s| {
                    s.quit();
                }),
        );
        self.sui.run();

        
        Ok(())
    }
}

pub struct TextDriver<'game> {
    pub progress: Progress,
    game: &'game Game,
}

impl<'game> TextDriver<'game> {
    pub fn new(game: &'game Game) -> Self {
        TextDriver {
            progress: Progress {
                scene: SceneIdentifier("".to_string()),
                character: Character {
                    stats: HashMap::new(),
                    inventory: HashSet::new(),
                    achievements: HashSet::new(),
                    state: State::Playing,
                },
            },
            game,
        }
    }
    fn view(&mut self, view: &SceneView) {
        println!("{}", view.description);
    }
    fn get_choice(view: &SceneView) -> Option<MenuItemIdentifier> {
        for (id, description) in &view.menu {
            println!("{}: {}", id.0, description);
        }
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        if choice.is_empty() || choice == "__QUIT" {
            None
        } else if view.menu.contains_key(&MenuItemIdentifier(choice.to_string())) {
            Some(MenuItemIdentifier(choice.to_string()))
        } else {
            println!("Invalid choice");
            Self::get_choice(view)
        }
    }
    
}

impl<'game> Driver<'game> for TextDriver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>> {
        let mut view = SceneView {
            description: String::new(),
            menu: HashMap::new(),
        };
        self.game.initialize(&mut self.progress, &mut view);
        loop {
            self.view(&view);
            if let Some(choice) = Self::get_choice(&view) {
                if !self.game.choose(&mut self.progress, &choice, &mut view)? {
                    break;
                }
            } else {
                break;
            }
        }
        if self.progress.character.state == State::Won {
            println!("Congratulations! You won!");
        } else if self.progress.character.state == State::Lost {
            println!("Sorry, you lost.");
        }
        Ok(())
    }
}


