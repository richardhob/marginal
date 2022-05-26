mod tests {
    use crate::one::something;

    #[test]
    fn test_fail() {
        let i = something();
        assert_eq!(i, 1);
    }
}
