pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn string_splits() {
        let expectations:[String; 2] = [String::from("hello"),
            String::from("world"),
        ];

        let results = [String::from("fail"),
            String::from("sadge"),
        ];

        assert_eq!(results, expectations);
    }

}
