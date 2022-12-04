pub fn get_answer() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_answer() {
        let result = get_answer();
        assert_eq!(result, 42);
    }
}
