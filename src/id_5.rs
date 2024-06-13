use super::Solution;

pub struct Id5 {

}

impl Solution for Id5 {
    fn solve() {
        println!("Id5 solution: {}", Id5::create_product(10));
    }
}

impl Id5 {
    fn create_product(range_lim: u128) -> u128 {
        let mut final_product = 1;
        let mut composing_values: Vec<u128> = vec![];
        let mut is_divisible: bool = false;

        // Before multiplying a value by the final product, we must perform 2 tests:
        //  1. Test whether the actual product is divisible by the value we are testing
        //
        //  2. If the previous test is false, then we test whether there is a number
        //      that are a divisor of our number. If there is, we multiply our product
        //      by the result of divind the actual number by its divisor and we add the
        //      number to our list
        //
        for val in 1..=range_lim {
            if (final_product % val) != 0 {
                // Composing values should be sorted at any point in the iteration, right?
                for inner_val in composing_values.clone().reverse().iter() {
                    if (val % inner_val) == 0 {
                        final_product *= val / inner_val;
                        println!("divisible: {:?}", val / inner_val);
                        is_divisible = true;
                        break;
                    } 
                }
                if !is_divisible {
                    final_product *= val;
                }
                is_divisible = false;
                composing_values.push(val);

                println!("{:?}", composing_values);
            }
        }

        return final_product;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is1001_palindrome() {
    }
}
