// main_test.rs

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }
}