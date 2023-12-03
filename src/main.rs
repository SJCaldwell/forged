use std::env;

mod dice_input;
use dice_input::DiceInput;

fn main() {
    let args: Vec<String> = env::args().collect();

    //First argument is path to the binary
    if args.len() > 1 {
        let input_string = &args[1];
        match DiceInput::parse(input_string) {
            Ok(dice_input) => {
                println!("{}", dice_input.roll());
            }
            Err(e) => {
                println!("Error parsing: {}", e);
            }
        }
    } else {
        println!("No input received");
    }
}
