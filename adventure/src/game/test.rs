use serde::{Deserialize, Serialize};
use crate::game::{
    ObjectIdentifier,
    AchievementIdentifier,
    Character,
    CommodityIdentifier,
};

#[derive(Deserialize,Serialize)]
pub struct Test {
    pub stat_greater: Option<(String, f32)>,
    pub stat_less: Option<(String, f32)>,
    pub commodity_greater: Option<(CommodityIdentifier, u32)>,
    pub commodity_less: Option<(CommodityIdentifier, u32)>,
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
            Test { commodity_greater: Some((commodity, value)), .. } => write!(f, "{:?} > {}", commodity, value),
            Test { commodity_less: Some((commodity, value)), .. } => write!(f, "{:?} < {}", commodity, value),
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
            Test { commodity_greater: Some((commodity, value)), .. } => {
                let ret = character.inventory.commodities.get(commodity).is_some_and(|v| v > value);
                log::debug!("Testing {:?} > {} = {}", commodity, value, ret);
                ret
            }
            Test { commodity_less: Some((commodity, value)), .. } => character.inventory.commodities.get(commodity).is_some_and(|v| v < value),
            Test { inventory_contains: Some(object), .. } => {
                let ret = character.inventory.objects.contains_key(object);
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

