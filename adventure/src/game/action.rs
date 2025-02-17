use serde::{Deserialize, Serialize};
use crate::game::{
    ObjectIdentifier,
    AchievementIdentifier,
    CommodityIdentifier,
    cache::CacheIdentifier,
    location::LocationIdentifier,
    progress::Progress,
};
#[derive(Debug,Deserialize,Serialize,Default,PartialEq)]
pub enum Action {
    #[default]
    None,
    AddStat(String, f32),
    TakeObject(LocationIdentifier, CacheIdentifier, ObjectIdentifier),
    PlaceObject(LocationIdentifier, CacheIdentifier, ObjectIdentifier),
    TakeCommodity(LocationIdentifier, CacheIdentifier, CommodityIdentifier, i32),
    UseCommodity(CommodityIdentifier, i32),
    PlaceCommodity(LocationIdentifier, CacheIdentifier, CommodityIdentifier, i32),
    AddAchievement(AchievementIdentifier),
    Win,
    Lose,
    Quit,
}

impl Action {
    pub fn act(&self, progress: &mut Progress) -> bool {
        match self {
            Action::None => true,
            Action::Quit => false,
            Action::AddStat(stat, value) => {
                let val = progress.character.stats.get_mut(stat).unwrap();
                *val += value;

                true
            },
            Action::TakeObject(location, cache, object) => {
                log::debug!("Taking object {:?} from cache {:?} at location {:?}", object, cache, location);
                progress.take_object(location, cache, object)
            },
            Action::PlaceObject(location, cache, object) => {
                progress.place_object(location, cache, object)
            },
            Action::TakeCommodity(location, cache, commodity, count) => {
                progress.take_commodity(location, cache, commodity, *count)
            },
            Action::PlaceCommodity(location, cache, commodity, count) => {
                progress.place_commodity(location, cache, commodity, *count)
            },
            Action::UseCommodity(commodity, count) => {
                progress.character.inventory.remove_commodity(commodity, *count)
            },
            Action::AddAchievement(achievement) => {
                progress.character.achievements.insert(achievement.clone());
                true
            },
            Action::Win => {
                progress.character.state = crate::game::State::Won;
                true
            },
            Action::Lose => {
                progress.character.state = crate::game::State::Lost;
                true
            },
        }
    }
}

