use std::collections::HashMap;

#[path="trev_lib/mod.rs"]
#[macro_use]
mod trev_lib;
use self::trev_lib::*;
impl_trev_lib!(); // bring in my HashMap extension

import_all!(point, "point.rs");

mod cardinal_direction {
    pub const NORTH: i8 = 0;
    pub const EAST: i8 = 1;
    pub const SOUTH: i8 = 2;
    pub const WEST: i8 = 3;
}

#[derive(Default)]
pub struct Player {
    pub position: Point,
    pub first_dup_position: Point,
    direction: i8,
    map: HashMap<isize, HashMap<isize, bool>>,
}
impl Player {
    fn rotate(&mut self, direction: char) {
        match direction {
            'R' => self.direction += 1,
            'L' => self.direction -= 1,
            _ => {}
        }
        if self.direction > 3 {
            self.direction = 0;
        }
            else if self.direction < 0 {
                self.direction = 3;
            }
    }

    fn move_forward(&mut self, amount: isize) {
        for _ in 0..amount {
            match self.direction {
                cardinal_direction::NORTH => self.position.y += 1,
                cardinal_direction::SOUTH => self.position.y -= 1,
                cardinal_direction::EAST => self.position.x += 1,
                cardinal_direction::WEST => self.position.x -= 1,
                _ => {}
            }
            let map_val: &mut bool = self.map.ioi(self.position.x).ioi(self.position.y);
            if self.first_dup_position == Point::ZERO && *map_val {
                self.first_dup_position = self.position.clone();
            }
            *map_val = true;
        }
    }

    pub fn perform_action(&mut self, action: &str) {
        self.rotate(action.as_bytes()[0] as char);
        self.move_forward(action[1..].parse().unwrap());
    }
}