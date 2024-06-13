use super::Solution;

pub struct Id3 {
    
}

impl Solution for Id3 {
    fn solve() {
        println!("Id3 solution: {:?}", Id3::find_prime_factors(600851475143));
    }
}

impl Id3 {

    fn find_prime_factors(num: u128) -> Vec<u128> {
        let mut factors: Vec<u128> = vec![];

        for val in 2..(num as f64).sqrt() as u128 {
            if (num % val) == 0 && val.is_prime() {
                factors.push(val);
            }
        }

        factors.sort();
        factors
    }
}

trait PrimeTest {
    fn is_prime(self) -> bool;
}


impl PrimeTest for u128 {
    // TODO: there are cooler and more efficient algorithms to do this. Try 
    // them some time in the future
    fn is_prime(self) -> bool {
        for val in 2..=(self as f64).sqrt() as u128 {
            if (self % val) == 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prime_factors() {
        let factors_13195 = vec![5, 7, 13, 29];
        assert_eq!(Id3::find_prime_factors(13195), factors_13195);
    }
}
