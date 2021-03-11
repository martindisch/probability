use rand::prelude::*;

pub fn run() {}

/// The stage with 3 doors.
#[derive(Clone, Copy)]
struct Stage {
    doors: [Door; 3],
}

impl Stage {
    fn new() -> Self {
        let mut doors = [Door::Closed(Content::Goat); 3];

        let car_door = (0..3).choose(&mut rand::thread_rng()).unwrap();
        doors[car_door] = Door::Closed(Content::Car);

        Self { doors }
    }
}

/// A door on the stage.
#[derive(Clone, Copy)]
enum Door {
    Closed(Content),
    Open(Content),
}

/// The content behind a door.
#[derive(Clone, Copy)]
enum Content {
    Goat,
    Car,
}
