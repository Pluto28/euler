use super::Solution;

pub struct Id1 {
    
}

impl Solution for Id1 {
    fn solve() {
        let summation = Id1::sum_mult_5_or_3(1000);
        println!("{:>30}", format!("Id1 iterative: {}", summation));
    }
}

impl Id1 {
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
        assert_eq!(Id1::sum_mult_5_or_3(10), 23);
    }
}
