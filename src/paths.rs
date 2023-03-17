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

use std::fs;
use std::path::PathBuf;

fn get_dir(dirtype: Option<PathBuf>) -> PathBuf {
    match dirtype {
        Some(path) => {
            let mut path = path;
            path.push("leafish");
            if !path.exists() {
                fs::create_dir_all(path.clone()).unwrap();
            }
            path
        }
        None => panic!("Unsupported platform"),
    }
}

pub fn get_config_dir() -> PathBuf {
    get_dir(dirs::config_dir())
}

pub fn get_cache_dir() -> PathBuf {
    get_dir(dirs::cache_dir())
}

pub fn get_data_dir() -> PathBuf {
    get_dir(dirs::data_dir())
}
