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

use bevy_ecs::prelude::*;
use parking_lot::RwLock;
use std::sync::Arc;

// System labels to enforce a run order of our systems
#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemExecStage {
    PreClearRemoveHandling, // TODO: This is a mess, clean it up as soon as bevy fixed the various remove detection issues!
    PreNormal,
    Normal,
    Render,
    RemoveHandling,
}

#[derive(Default)]
pub struct Manager {
    pub world: World,
    pub schedule: Arc<RwLock<Schedule>>,
    pub entity_schedule: Arc<RwLock<Schedule>>,
}
