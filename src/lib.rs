
#[cfg(test)]
mod tests {
    #[test]
    fn hello_test() {
        let some_string = "Hello, test!";

        assert_eq!(some_string, String::from("Hello, test!"))
    }

    #[test]
    fn some_math() {
        let result = 2 + 2;

        assert_eq!(result, 4)
    }

    #[test]
    fn more_math() {
        let result = 4 + 2;

        assert_eq!(result, 6)
    }

    #[test]
    fn multiply() {
        let result = 4 * 2;

        assert_eq!(result, 8)
    }

    #[test]
    fn divide() {
        let result = 4 / 2;

        assert_eq!(result, 2)
    }
}

// Some unimportant change
