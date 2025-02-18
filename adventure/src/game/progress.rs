use std::collections::{HashMap,HashSet};
use serde::{Deserialize, Serialize};
use super::{
    Game,
    ObjectIdentifier,
    CommodityIdentifier,
    cache::{Cache, CacheIdentifier},
    scene::SceneIdentifier,
    location::LocationIdentifier,
    Character,
};
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Progress {
    pub scene: SceneIdentifier,
    pub character: Character,
    pub caches: HashMap<LocationIdentifier, HashMap<CacheIdentifier, Cache>>,
    pub scene_history: HashSet<SceneIdentifier>,
    pub location_history: HashSet<LocationIdentifier>,
}
impl Progress {
    pub fn new(game: &Game) -> Self {
        let mut hm = HashMap::new();
        for (lid, location) in &game.locations {
            let mut caches = HashMap::new();
            for (cid, cache) in &location.caches {
                caches.insert(cid.clone(), cache.clone());
            }
            
            hm.insert(lid.clone(), caches);
        }
        Progress {
            scene: game.start_scene.clone(),
            character: game.character.clone(),
            caches: hm,
            scene_history: HashSet::new(),
            location_history: HashSet::new(),
        }
    }
    pub fn take_object(&mut self, location: &LocationIdentifier, cache: &CacheIdentifier, object: &ObjectIdentifier) -> bool {
        if let Some(caches) = self.caches.get_mut(location) {
            if let Some(cache) = caches.get_mut(cache) {
                return self.character.inventory.take_object(cache, object);
            }
        }
        false
    }
    pub fn place_object(&mut self, location: &LocationIdentifier, cache: &CacheIdentifier, object: &ObjectIdentifier) -> bool {
        if let Some(caches) = self.caches.get_mut(location) {
            if let Some(cache) = caches.get_mut(cache) {
                return cache.take_object(&mut self.character.inventory, object);
            }
        }
        false
    }
    pub fn take_commodity(&mut self, location: &LocationIdentifier, cache: &CacheIdentifier, commodity: &CommodityIdentifier, count: i32) -> bool {
        if let Some(caches) = self.caches.get_mut(location) {
            if let Some(cache) = caches.get_mut(cache) {
                return self.character.inventory.take_commodity(cache, commodity, count);
            }
        }
        false
    }
    pub fn place_commodity(&mut self, location: &LocationIdentifier, cache: &CacheIdentifier, commodity: &CommodityIdentifier, count: i32) -> bool {
        if let Some(caches) = self.caches.get_mut(location) {
            if let Some(cache) = caches.get_mut(cache) {
                return cache.take_commodity(&mut self.character.inventory, commodity, count);
            }
        }
        false
    }
}

