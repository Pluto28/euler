use super::Solution;

pub struct Id4 {

}

impl Solution for Id4 {
    fn solve() {
        println!("Id4 solution: {}", Id4::greatest_palindrome_digits(3));
    }
}

impl Id4 {
    fn greatest_palindrome_digits(digits: u8) -> u128 {
        let mut gt_prod: u128 = 0;
        let grt_possible_val = 10_u128.pow(digits as u32) - 1;
        let lwst_possible_val = 10_u128.pow((digits - 1) as u32);

        for lower_vals in lwst_possible_val..=grt_possible_val {
            for upper_vals in lwst_possible_val..=grt_possible_val {
                if (lower_vals * upper_vals).is_palindrome() {
                    gt_prod = lower_vals * upper_vals;
                }
            }

        }
        gt_prod
    }

}

trait IsPalindrome {
    fn is_palindrome(self) -> bool;
}

impl IsPalindrome for u128 {
    fn is_palindrome(self) -> bool {
        let val_str = self.to_string(); 
        let half_low = (val_str.len() as f64 / 2_f64).floor() as usize;
        let half_high = (val_str.len() as f64 / 2_f64).ceil() as usize;

        let first_slice = &val_str[0..half_low];
        let second_slice = &val_str[half_high..val_str.len()].
            chars().
            rev().
            collect::<String>();

        first_slice.eq(second_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is1001_palindrome() {
        let val = 1001;

        assert_eq!(val.is_palindrome(), true);
    }

    #[test]
    fn is10101_palindrome() {
        let val = 10101;

        assert_eq!(val.is_palindrome(), true);
    }

    #[test]
    fn is1313_palindrome() {
        let val = 1313;

        assert_eq!(val.is_palindrome(), false);
    }
}
