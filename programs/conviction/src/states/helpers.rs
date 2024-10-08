pub fn calculate_lock_duration(lock_period: u8) -> i64 {
    const SECONDS_PER_WEEK: i64 = 7 * 24 * 60 * 60;
    (lock_period as i64) * SECONDS_PER_WEEK
}

pub fn calculate_conviction_multiplier(lock_period: u8) -> u8 {
    let max_doublings = 6;
    let mut multiplier = 1;
    let mut current_period = 1;

    for _ in 0..max_doublings {
        if lock_period >= current_period {
            multiplier += 1;
            current_period *= 2;
        } else {
            break;
        }
    }

    multiplier
}

pub fn calculate_voting_power(amount: u64, conviction_multiplier: u8) -> u64 {
    amount * (conviction_multiplier as u64)
}