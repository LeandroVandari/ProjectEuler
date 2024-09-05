use std::io::Write;
mod problems;
use problems::rust;

fn main() {
    print!("Welcome to the Project Euler problem solver app! To begin, choose the problem you'd like to display: ");
    let _ = std::io::stdout().flush();

    loop {
        let problem_num = loop {
            let mut problem_num = String::new();
            std::io::stdin()
                .read_line(&mut problem_num)
                .expect("Problem to read line!");
            problem_num.pop();
            match problem_num.parse::<u32>() {
                Err(e) => {
                    println!("Could not parse problem number: {e};")
                }
                Ok(num) => break num,
            }
        };

        println!(
            "The answer to this problem is: {}",
            rust::problem_map.get(&problem_num).unwrap()()
        );
    }
}
