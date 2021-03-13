use rand::prelude::*;

pub fn run() {
    let mut test_runner = TestRunner::new(vec![
        Box::from(NonSwitchingContestant),
        Box::from(SwitchingContestant),
    ]);

    let results = test_runner.compare_strategies(1_000_000);
    println!("{:#?}", results);
}

struct TestRunner {
    contestants: Vec<Box<dyn Contestant>>,
}

impl TestRunner {
    /// Initializes a new `Testrunner`.
    fn new(contestants: Vec<Box<dyn Contestant>>) -> Self {
        Self { contestants }
    }

    /// Compares the contestant strategies against each other.
    ///
    /// # Arguments
    /// * `runs` - The number of runs to do.
    fn compare_strategies(&mut self, runs: u32) -> Vec<TestResult> {
        let mut wins: Vec<u32> = vec![0; self.contestants.len()];

        for _ in 0..runs {
            let stage = Stage::new();

            for (i, contestant) in self.contestants.iter().enumerate() {
                // Let contestant make their first choice
                let first_choice = contestant.first_choice();

                // Open another door containing a goat
                let door_to_open = stage
                    .doors
                    .iter()
                    .enumerate()
                    .filter(|(i, &c)| *i != first_choice && c == Door::Goat)
                    .map(|(i, _)| i)
                    .next()
                    .unwrap();

                // Let contestant make their final choice
                let final_choice = match (
                    contestant.switch_door(),
                    first_choice,
                    door_to_open,
                ) {
                    (false, _, _) => first_choice,
                    (true, 0, 1) => 2,
                    (true, 0, 2) => 1,
                    (true, 1, 0) => 2,
                    (true, 1, 2) => 0,
                    (true, 2, 0) => 1,
                    (true, 2, 1) => 0,
                    _ => unreachable!(),
                };

                // Determine result
                if stage.doors[final_choice] == Door::Car {
                    wins[i] += 1;
                }
            }
        }

        wins.iter()
            .enumerate()
            .map(|(i, &total)| TestResult {
                contestant_strategy: self.contestants[i].name(),
                number_of_wins: total,
            })
            .collect()
    }
}

#[derive(Debug)]
struct TestResult {
    contestant_strategy: &'static str,
    number_of_wins: u32,
}

/// The stage with 3 doors.
struct Stage {
    doors: [Door; 3],
}

impl Stage {
    fn new() -> Self {
        let mut doors = [Door::Goat; 3];

        let car_door = (0..3).choose(&mut rand::thread_rng()).unwrap();
        doors[car_door] = Door::Car;

        Self { doors }
    }
}

/// The content behind a door.
#[derive(Clone, Copy, PartialEq)]
enum Door {
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
