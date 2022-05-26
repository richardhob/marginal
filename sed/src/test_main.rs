mod tests {
    use crate::one::something;

    #[test]
    fn test_fail() {
        something();
        assert_eq!(1, 0);
    }
}
