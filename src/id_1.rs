use super::Solution;

pub struct Id1 {
    
}

impl Solution for Id1 {
    fn solve() {
        let mut summation = Id1::fact_sum_even(4_000_000);
        println!("{:>30}", format!("Id1 iterative: {}", summation));

        summation = Id1::fact_sum_even_formula(4_000_000);
        println!("{:>30}", format!("Id1 iterative formula: {}", summation));
    }
}

impl Id1 {
    fn fact_sum_even(limit: u64) -> u64 {
        let mut summation: u64 = 0;
        let mut fib_first: u64 = 1;
        let mut fib_second: u64 = 2;
        let mut fib_actual: u64 = 2;

        loop {
            if fib_actual > limit {
                break;
            }

            if (fib_actual % 2) == 0 {
                //println!("{}", fib_actual);
                summation = summation + fib_actual; 
            }

            fib_actual = fib_first + fib_second;
            fib_first = fib_second;
            fib_second = fib_actual;
        }

        return summation;
    }
    
    // I don't like the way i write, so i'll use comments to tell a little story
    // as practice. No one should read this in a reasonable amount of time, so 
    // when they do i'll have a decent excuse and/or will be able to laugh at
    // the situation
    fn fact_sum_even_formula(val: u64) -> u64 {
        let golden_ratio: f64 = (1.0+5.0_f64.sqrt()) / 2.0;
        let mut fib_num = 0;
        let mut index = 0;
        let mut sum = 0;

        loop {
            fib_num = ((golden_ratio.powf(index as f64)) / 5.0_f64.sqrt()).round() as u64;
            if fib_num > val {
                break;
            } else if (fib_num % 2) == 0 {
                sum += fib_num;
            }
            index += 1;
        }
    
        return sum;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_to_10() {
        assert_eq!(Id1::fact_sum_even(89), 44);
    }

    #[test]
    fn test_up_to_10_rec() {
        assert_eq!(Id1::fact_sum_even_formula(90), 44);
    }
}
