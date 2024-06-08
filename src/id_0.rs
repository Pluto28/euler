use super::Solution;

pub struct Id0 {
    
}

impl Solution for Id0 {
    fn solve() {
        let summation = Id0::sum_mult_5_or_3(1000);
        println!("{:>30}", format!("Id0 iterative: {}", summation));
    }
}

impl Id0 {
    fn sum_mult_5_or_3(limit: u64) -> u64 {
        let mut summation: u64 = 0;

        for val in 0..limit {
            if (val % 3) == 0 || (val % 5) == 0 {
                summation = summation + val; 
            }
        }

        return summation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_to_10() {
        assert_eq!(Id0::sum_mult_5_or_3(10), 23);
    }
}
