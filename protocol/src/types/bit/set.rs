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

#[derive(Clone, Debug)]
pub struct Set {
    data: Vec<u64>,
}

#[test]
fn test_set() {
    let mut set = Set::new(200);
    for i in 0..200 {
        if i % 3 == 0 {
            set.set(i, true)
        }
    }
    for i in 0..200 {
        if set.get(i) != (i % 3 == 0) {
            panic!("Fail")
        }
    }
}

impl Set {
    pub fn new(size: usize) -> Set {
        Set {
            data: vec![0; (size + 63) / 64],
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.data.resize((new_size + 63) / 64, 0);
    }

    pub fn capacity(&self) -> usize {
        self.data.len() * 64
    }

    pub fn set(&mut self, i: usize, v: bool) {
        if v {
            self.data[i >> 6] |= 1 << (i & 0x3F)
        } else {
            self.data[i >> 6] &= !(1 << (i & 0x3F))
        }
    }

    pub fn get(&self, i: usize) -> bool {
        (self.data[i >> 6] & (1 << (i & 0x3F))) != 0
    }

    pub fn includes_set(&self, other: &Set) -> bool {
        for (a, b) in self.data.iter().zip(&other.data) {
            if a & b != *b {
                return false;
            }
        }
        true
    }

    pub fn or(&mut self, other: &Set) {
        for (a, b) in self.data.iter_mut().zip(&other.data) {
            *a |= *b;
        }
    }
}
