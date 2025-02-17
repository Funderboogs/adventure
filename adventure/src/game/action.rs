use serde::{Deserialize, Serialize};
use crate::game::{
    ObjectIdentifier,
    AchievementIdentifier,
    Character,
    State,
    CommodityIdentifier,
};
#[derive(Debug,Deserialize,Serialize,Default)]
pub enum Action {
    #[default]
    None,
    AddStat(String, f32),
    AddToInventory(ObjectIdentifier),
    AddCommodity(CommodityIdentifier, i32),
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
            Action::AddCommodity(commodity, amount) => {
                let commodity_amount = character.commodities.entry(commodity.clone()).or_insert(0);
                if *amount < 0 {
                    *commodity_amount -= amount.unsigned_abs();
                } else {
                    *commodity_amount += *amount as u32;
                }
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

