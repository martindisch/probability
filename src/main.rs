fn main() {
    println!("Hello, world!");
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
