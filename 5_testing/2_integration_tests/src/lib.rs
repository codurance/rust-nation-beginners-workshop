pub fn is_leap_year(year: u64) -> bool {
    is_divisible_by(year, 400) ||
            is_divisible_by(year, 4)
}

fn is_divisible_by(year: u64, divisor: u64) -> bool {
    year / divisor == 0
}

fn is_not_divisible_by(year: u64, divisor: u64) -> bool {
    !is_divisible_by(year, divisor)
}
