// The actual function
pub fn square(x: i32) -> i32 {
    x * x
}

// The function that we want to conditionally include just for testing purposes
#[cfg(test)]
pub fn square_ref(x: &i32) -> i32 {
    square(*x)
}

// All of the code in this module will not be included in the user-facing
// compiled code because the no-arg `test` configuration option will only
// be set when unit- and integration-tests are running.
#[cfg(test)]
mod tests {
    use super::*;
    
    // This include will not affect user-facing compiled code becauase it is in
    // the testing-only module.
    use std::vec::Vec;

    #[test]
    fn square_test() {
        assert_eq!(4, square(2));
        assert_eq!(4, square(-2));
        assert_eq!(0, square(0));
        assert_eq!(1234321, square(1111));
    }

    // This test will compile, even though it imports `Vec`, which doesn't 
    // exist in the user-facing code, only because it is only called during 
    // `cargo test`. `cargo test` sets the no-arg `test` configuration option
    // while the tests are running.
    #[test]
    fn square_ref_test() {
        let nums: Vec<i32> = vec![-100, -10, -5, -3, -1, 0, 1, 4, 11, 19, 100];
        let exp_squares = vec![10000, 100, 25, 9, 1, 0, 1, 16, 121, 361, 10000];
        let act_squares = nums.iter().map(square_ref).collect::<Vec<i32>>();
        assert_eq!(act_squares, exp_squares);
    }
}
