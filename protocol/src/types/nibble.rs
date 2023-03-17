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

#[derive(Clone)]
pub struct Array {
    pub data: Vec<u8>,
}

impl Array {
    pub fn new(size: usize) -> Self {
        Array {
            data: vec![0; (size + 1) >> 1],
        }
    }

    pub fn new_def(size: usize, def: u8) -> Self {
        let def = (def & 0xF) | ((def & 0xF) << 4);
        Array {
            data: vec![def; (size + 1) >> 1],
        }
    }

    pub fn get(&self, idx: usize) -> u8 {
        let val = self.data[idx >> 1];
        if idx & 1 == 0 {
            val & 0xF
        } else {
            val >> 4
        }
    }

    pub fn set(&mut self, idx: usize, val: u8) {
        let i = idx >> 1;
        let old = self.data[i];
        if idx & 1 == 0 {
            self.data[i] = (old & 0xF0) | (val & 0xF);
        } else {
            self.data[i] = (old & 0x0F) | ((val & 0xF) << 4);
        }
    }
}
