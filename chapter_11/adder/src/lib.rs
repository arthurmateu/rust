pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(add(2, 2), 4);
    }
    #[test]
    #[should_panic]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }
}
