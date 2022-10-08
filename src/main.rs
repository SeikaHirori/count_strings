use logic;
use std::io;

fn main() {
    println!("Let's start the program! :3");

    let mut index_loop = 1;
    
    loop {
        

        println!("Type a sentence, and we'll count it for you :3");
        println!("To exit program, type: exit()");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let trimmed_input = user_input.trim();

        // RFER 04
        match trimmed_input {
            "exit()" => {
                break
            },            
            _ => {

                let word_list = logic::split_strings(user_input);
                let word_count = logic::count_words(word_list);
                println!("Word count: {}", word_count);
            },
        }
        println!("\n===== Loop {} =====", index_loop);
        index_loop += 1;
    }




    println!("Ending program! :3")
}
