// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    println!("{}",is_even(2));
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2), "2 should be even");
        assert!(!is_even(3), "3 should not be even");
        assert!(is_even(4), "4 should be even");
        assert!(!is_even(5), "5 should not be even");
    }
}
