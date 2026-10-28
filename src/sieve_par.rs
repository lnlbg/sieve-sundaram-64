//! Parallel Algorithm.

use crate::bitset::BitSet;
use std::thread;

/// Computes primes within a given interval in using 2 threads.
///
/// # Arguments
///
/// * 'limit_down' - The lower bound of the interval (inclusive).
/// * 'limit_up' - The upper bound of the interval (inclusive).
///
/// # Errors
///
/// Returns an 'Err' string if:
/// * 'limit_up' or 'limit_down' exceed the safe maximum threshold.
/// * 'limit_up' or 'limit_down' are less than 5.
/// * 'limit_down' is greater than 'limit_up'.
/// * The interval range ('limit_up - limit_down') exceeds 1,000,000,000.
pub fn sundaram_sieve_par(limit_down: u64, limit_up: u64) -> Result<Vec<u64>, String> {
    if limit_up > u64::MAX - 58
        || limit_down > u64::MAX - 58
        || limit_up < 5
        || limit_down < 5
        || limit_down > limit_up
        || limit_up - limit_down > 1_000_000_000
    {
        return Err("Invalid limits provided !".to_string());
    }

    let min_k: u64 = ((limit_down + 4) / 6).max(1);
    let max_k: u64 = ((limit_up - 5) / 6) + 1;

    let max_i = (limit_up.isqrt() + 1) / 6;
    let range_k = (max_k - min_k + 1) as usize;

    let (composite_plus, composite_minus) = thread::scope(|s| {
        let handle_plus = s.spawn(move || {
            let mut local_plus = BitSet::new(range_k);

            for i in 1..=max_i {
                let denom1 = 6 * i - 1;
                let j_max1 = (max_k + i) / denom1;
                let j_min1 = ((min_k + i + denom1 - 1) / denom1).max(i);

                for j in j_min1..=j_max1 {
                    let k = 6 * i * j - i - j;
                    local_plus.set((k - min_k) as usize);
                }

                let denom2 = 6 * i + 1;
                let j_max2 = max_k.saturating_sub(i) / denom2;
                let j_min2 = ((min_k.saturating_sub(i) + denom2 - 1) / denom2).max(i);

                for j in j_min2..=j_max2 {
                    let k = 6 * i * j + i + j;
                    local_plus.set((k - min_k) as usize);
                }
            }

            local_plus
        });

        let handle_minus = s.spawn(move || {
            let mut local_minus = BitSet::new(range_k);

            for i in 1..=max_i {
                let denom3 = 6 * i - 1;
                let j_max3 = max_k.saturating_sub(i) / denom3;
                let j_min3 = ((min_k.saturating_sub(i) + denom3 - 1) / denom3).max(i);

                for j in j_min3..=j_max3 {
                    let k = 6 * i * j + i - j;
                    local_minus.set((k - min_k) as usize);
                }

                let denom4 = 6 * i + 1;
                let j_max4 = (max_k + i) / denom4;
                let j_min4 = ((min_k + i + denom4 - 1) / denom4).max(i);

                for j in j_min4..=j_max4 {
                    let k = 6 * i * j - i + j;
                    local_minus.set((k - min_k) as usize);
                }
            }

            local_minus
        });

        let p = handle_plus.join().unwrap();
        let m = handle_minus.join().unwrap();
        (p, m)
    });

    let mut primes = Vec::new();

    for k in min_k..=max_k {
        let idx = (k - min_k) as usize;

        let p_minus = 6 * k - 1;
        if p_minus >= limit_down && p_minus <= limit_up && !composite_minus.is_set(idx) {
            primes.push(p_minus);
        }

        let p_plus = 6 * k + 1;
        if p_plus >= limit_down && p_plus <= limit_up && !composite_plus.is_set(idx) {
            primes.push(p_plus);
        }
    }

    Ok(primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_down_greater_than_up() {
        let result = sundaram_sieve_par(20, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_interval_too_large_parallel() {
        let result = sundaram_sieve_par(5, 5 + 1_000_000_001);
        assert!(result.is_err());
    }
}
