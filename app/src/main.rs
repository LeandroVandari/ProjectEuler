use std::io::Write;

fn main() {
    print!("Welcome to the Project Euler problem solver app! To begin, choose the problem you'd like to display: ");
    let _ = std::io::stdout().flush();

    let problem_num = loop {
        let mut problem_num = String::new();
        std::io::stdin().read_line(&mut problem_num).expect("Problem to read line!");
        problem_num.pop();
        dbg!(&problem_num);
        match problem_num.parse::<u32>() {
            Err(e) => {println!("Could not parse problem number: {e};")},
            Ok(num) => break num,
        }
    };

    println!("Alright! Now, ")


    
}
