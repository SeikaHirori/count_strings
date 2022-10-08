pub fn split_strings(user_string:String) -> Vec<String> {
    /* _Objective_
        - Breakdown what is happening from RFER 03
    */

    let splited = user_string.split_whitespace();
    
    let mapped = splited.map(str::to_string);
    let output = mapped.collect();
    
    output
}

pub fn count_words(word_list:Vec<String>) -> usize {
    let amount = word_list.len();

    amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_splits() {
        let expectations:[String; 2] = [String::from("hello"),
            String::from("world"),
        ];

        let user_string = String::from("hello world");

        let results = split_strings(user_string);

        assert_eq!(results, 
            expectations,
            "The values are not the same. Results was '{:?}', \n but expected '{:?}' instead.",
            results,
            expectations,
        ); // RFER 02
    }

    #[test]
    fn test_counting_words() {
        let user_string = String::from("hello world");
        let word_list = split_strings(user_string);
        
        let results = count_words(word_list);

        assert_eq!(results, 2)

    }

}
