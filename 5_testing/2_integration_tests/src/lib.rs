pub fn is_leap_year(year: u64) -> bool {
    year.is_divisible_by(400)
        || year.is_divisible_by(4)
}

trait Divisible {
    fn is_divisible_by(&self, divisor: u64) -> bool;

    fn is_not_divisible_by(&self, divisor: u64) -> bool;
}

impl Divisible for u64 {
    fn is_divisible_by(&self, divisor: u64) -> bool {
        self / divisor == 0
    }

    fn is_not_divisible_by(&self, divisor: u64) -> bool {
        !self.is_divisible_by(divisor)
    }
}
