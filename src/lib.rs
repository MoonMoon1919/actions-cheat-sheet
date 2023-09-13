
#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() {
        let some_string = "Hello, test!";

        assert_eq!(some_string, String::from("Hello, test!"))
    }

    #[test]
    fn some_math() {
        let sum = 2 + 2;

        assert_eq!(sum, 4)
    }

    #[test]
    fn more_math() {
        let sum = 4 + 2;

        assert_eq!(sum, 6)
    }

    #[test]
    fn multiply() {
        let sum = 4 * 2;

        assert_eq!(sum, 8)
    }
}

// Some unimportant change
