
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
}

// Some unimportant change
