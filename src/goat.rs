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

trait Contestant {
    /// Returns the index of the first chosen door (0, 1, 2).
    fn first_choice(&self) -> usize;

    /// Returns whether contestant changes selection after reveal.
    fn switch_door(&self) -> bool;

    /// Returns the name of this contestant strategy.
    fn name(&self) -> &'static str;
}

struct SwitchingContestant;

impl Contestant for SwitchingContestant {
    fn first_choice(&self) -> usize {
        (0..3).choose(&mut rand::thread_rng()).unwrap()
    }

    fn switch_door(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "Switching contestant"
    }
}

struct NonSwitchingContestant;

impl Contestant for NonSwitchingContestant {
    fn first_choice(&self) -> usize {
        (0..3).choose(&mut rand::thread_rng()).unwrap()
    }

    fn switch_door(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "Non-switching contestant"
    }
}
