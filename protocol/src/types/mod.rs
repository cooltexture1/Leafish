// This file is part of Leafish.
//
// Leafish is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Leafish is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Leafish. If not, see <https://www.gnu.org/licenses/>. 

mod metadata;
pub use self::metadata::*;

pub mod bit;
pub mod hash;
pub mod nibble;

use bevy_ecs::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    NotSet = -1,
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl GameMode {
    pub fn from_int(val: i32) -> GameMode {
        match val {
            3 => GameMode::Spectator,
            2 => GameMode::Adventure,
            1 => GameMode::Creative,
            0 => GameMode::Survival,
            -1 => GameMode::NotSet,
            _ => GameMode::Adventure,
        }
    }

    pub fn can_fly(&self) -> bool {
        matches!(*self, GameMode::Creative | GameMode::Spectator)
    }

    pub fn always_fly(&self) -> bool {
        matches!(*self, GameMode::Spectator)
    }

    pub fn noclip(&self) -> bool {
        matches!(*self, GameMode::Spectator)
    }

    pub fn can_interact_with_world(&self) -> bool {
        matches!(
            *self,
            GameMode::Creative | GameMode::Survival | GameMode::NotSet
        )
    }
}
