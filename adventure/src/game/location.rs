use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game::{
    Identifier,
    cache::{Cache, CacheIdentifier},
};

pub type LocationIdentifier = Identifier;

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct Location {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub short_description: String,
    #[serde(default)]
    pub caches: HashMap<CacheIdentifier, Cache>,
}

