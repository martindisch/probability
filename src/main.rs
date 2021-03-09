use rand::prelude::*;

fn main() {
    println!("Hello, world!");
}

struct TestRunner {
    rng: ThreadRng,
    booking_strategies: Vec<Box<dyn BookingStrategy>>,
}

impl TestRunner {
    /// Initializes a new `Testrunner`.
    fn new(booking_strategies: Vec<Box<dyn BookingStrategy>>) -> Self {
        Self {
            rng: rand::thread_rng(),
            booking_strategies,
        }
    }

    /// Compares the strategies against each other.
    ///
    /// # Arguments
    /// * `runs` - The number of runs to do.
    fn compare_strategies(&mut self, runs: u32) -> Vec<TestResult> {
        let mut cost_sums: Vec<u32> = vec![0; self.booking_strategies.len()];

        for _ in 0..runs {
            let lucky: bool = self.rng.gen();
            for (y, strategy) in self.booking_strategies.iter().enumerate() {
                let cost = strategy.calculate_cost(lucky);
                cost_sums[y] += cost;
            }
        }

        cost_sums
            .iter()
            .enumerate()
            .map(|(i, sum)| TestResult {
                strategy_name: self.booking_strategies[i].name(),
                average_cost: f64::from(*sum) / f64::from(runs),
            })
            .collect()
    }
}

#[derive(Debug)]
struct TestResult {
    strategy_name: &'static str,
    average_cost: f64,
}

trait BookingStrategy {
    /// Returns the cost of a lucky or unlucky booking.
    ///
    /// # Arguments
    /// * `lucky` - Whether you get the cheap option or not.
    fn calculate_cost(&self, lucky: bool) -> u32;

    /// Returns the name of this booking strategy.
    fn name(&self) -> &'static str;
}

struct WinterBooking;

impl BookingStrategy for WinterBooking {
    fn calculate_cost(&self, _lucky: bool) -> u32 {
        500
    }

    fn name(&self) -> &'static str {
        "Winter booking"
    }
}

struct SummerBooking;

impl BookingStrategy for SummerBooking {
    fn calculate_cost(&self, lucky: bool) -> u32 {
        match lucky {
            true => 250,
            false => 1000,
        }
    }

    fn name(&self) -> &'static str {
        "Summer booking"
    }
}
