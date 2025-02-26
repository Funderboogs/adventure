use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{
    Game,
    Identifier,
    Character,
    test::Test,
    action::Action,
    location::{LocationIdentifier, Location},
    progress::Progress,
    cache::{Cache, CacheIdentifier},
};

pub type SceneIdentifier = Identifier;
pub type MenuItemIdentifier = Identifier;

#[derive(Debug,Deserialize,Serialize)]
pub struct MenuItem {
    pub description: String,
    pub test: Option<Test>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub next_scene: Option<SceneIdentifier>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Scene {
    #[serde(default)]
    pub location: LocationIdentifier,
    pub description: String,
    #[serde(default)]
    pub short_description: String,
    pub menu: HashMap<MenuItemIdentifier, MenuItem>,
}

impl Scene {
    pub fn render_for(&self, game: &Game, progress: &Progress) -> tera::Result<String> {
        let mut renderer = tera::Tera::default();

        #[derive(Serialize)]
        struct RenderContext<'a> {
            character: &'a Character,
            scene: &'a Scene,
            location: &'a Location,
            caches: &'a HashMap<CacheIdentifier, Cache>,
            progress: &'a Progress,
            game: &'a Game,
        }
        let location = match game.locations.get(&self.location) {
            Some(location) => location,
            None => &Location::default(),
        };
        let caches = match progress.caches.get(&self.location) {
            Some(caches) => caches,
            None => &HashMap::new(),
        };
        let mut context = tera::Context::new();
        if !caches.is_empty() {
            context.extend(tera::Context::from_serialize(caches).unwrap());
        }
        context.extend(tera::Context::from_serialize(self).unwrap());
        context.extend(tera::Context::from_serialize(&progress.character).unwrap());
        context.extend(tera::Context::from_serialize(
            RenderContext {
                character: &progress.character,
                scene: self,
                location,
                caches,
                progress,
                game,
            }
        ).unwrap());
        let desc = if !self.short_description.is_empty() && progress.scene_history.contains(&progress.scene) {
            &self.short_description
        } else {
            &self.description
        };
        context.insert(
            "location_description", 
            if self.location.is_empty() {
                ""
            } else if progress.location_history.contains(&self.location) && !location.short_description.is_empty() {
                &location.short_description
            } else {
                &location.description
            },
        );
        
        renderer.render_str(desc, &context)
    }
}

#[derive(Debug,Deserialize,Serialize)]
pub struct SceneView {
    pub message: Option<String>,
    pub description: String,
    pub menu: HashMap<MenuItemIdentifier, String>,
}

