use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::game::{
    Identifier,
    ObjectIdentifier,
    Object,
    CommodityIdentifier,
};

pub type CacheIdentifier = Identifier;

#[derive(Debug,Default,Deserialize,Serialize,Clone)]
pub struct Cache {
    #[serde(default)]
    pub objects: HashMap<ObjectIdentifier, Object>,
    #[serde(default)]
    pub commodities: HashMap<CommodityIdentifier, u32>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            objects: HashMap::new(),
            commodities: HashMap::new(),
        }
    }
    pub fn take_object(&mut self, other: &mut Cache, id: &ObjectIdentifier) -> bool{
        if let Some(object) = other.objects.remove(id) {
            self.objects.insert(id.clone(), object);
            true
        } else {
            false
        }
    }
    pub fn remove_commodity(&mut self, id: &CommodityIdentifier, count: i32) -> bool {
        if let Some(commodity) = self.commodities.get_mut(id) {
            if *commodity >= count.unsigned_abs(){
                *commodity -= count.unsigned_abs();
                if *commodity == 0 {
                    self.commodities.remove(id);
                }
                return true
            }
        }
        false
    }
    pub fn add_commodity(&mut self, id: CommodityIdentifier, count: i32) {
        if let Some(commodity) = self.commodities.get_mut(&id) {
            *commodity += count.unsigned_abs();
        } else {
            self.commodities.insert(id, count.unsigned_abs());
        }
    }
    pub fn take_commodity(&mut self, other: &mut Cache, id: &CommodityIdentifier, count: i32) -> bool {
        if !other.remove_commodity(id, count) {
            return false
        }
        self.add_commodity(id.clone(), count);
        true
    }
}
