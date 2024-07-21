use std::io;
use deterministic_trigonometry::DTrig;

/* This is a basic command line trigonometry function calculator. It is designed as an example to show how to use the
deterministic-trigonometry crate. */
fn main() {
    // This initializes the DTrig struct which writes the pre-baked trig tables into memory.
    let d_trig = DTrig::initialize();

    // This variable allows the main loop to exit.
    let mut continue_running = true;

    // This is the main loop
    while continue_running == true {
        println!("Welcome to the trigonometry calculator!");
        println!("");

        // This helper function takes a code for the trig function and the input value from the user.
        let input_tuple = take_user_input();

        // Just for visual spacing.
        println!("");

        // This stores a code for the trig function. 1: sine | 2: cosine | 3: tangent | 4: arcsine | 5: arccosine | 6: arctangent
        let trig_function = input_tuple.0;

        // This stores the input for the trig function.
        let input_value = input_tuple.1;

        // This converts the input value to a faction over 1000.
        let fraction_tuple = ((input_value * 1000.0).round() as i32, 1000);

        // This provides the result based on the trig function and input value.
        match trig_function {
            1 => {
                println!("The answer is {}/{}", d_trig.sine(fraction_tuple).0, 1000);
            }
            2 => {
                println!("The answer is {}/{}", d_trig.cosine(fraction_tuple).0, 1000);
            }
            3 => {
                println!("The answer is {}/{}", d_trig.tangent(fraction_tuple).0, 1000);
            }
            4 => {
                println!("The answer is {}/{}", d_trig.arcsine(fraction_tuple).0, 1000);
            }
            5 => {
                println!("The answer is {}/{}", d_trig.arccosine(fraction_tuple).0, 1000);
            }
            6 => {
                println!("The answer is {}/{}", d_trig.arctangent(fraction_tuple).0, 1000);
            }
            _ => {
                println!("Input error. Please try again");
            }
        }

        // This asks the user if they want to do another calculation.
        continue_running = ask_continue_running();
    }
}

// This is the helper function to take user input for the calculator.

fn take_user_input() -> (i32, f32) {
    // This holds the trig function code.
    let mut trig_function: i32 = 0;

    // This holds the input value.
    let mut input_value: f32 = 0.0;

    let mut input_valid = false;

    // This takes and validates the trig function code.
    while input_valid == false {
        println!(
            "What function do you want? 1: sine | 2: cosine | 3: tangent | 4: arcsine | 5: arccosine | 6: arctangent"
        );
        println!("");

        let mut trig_function_string = String::new();
        match io::stdin().read_line(&mut trig_function_string) {
            Ok(_) => {}
            Err(_) => {
                println!("Input error. Try again.");
                println!("");
                continue;
            }
        }

        trig_function = match trig_function_string.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input error. Try again.");
                println!("");
                continue;
            }
        };

        if trig_function >= 1 && trig_function <= 6 {
            input_valid = true;
        } else {
            println!("Input error. Try again.");
            println!("");
            continue;
        }
    }

    input_valid = false;

    // This takes and validates the input value.
    while input_valid == false {
        println!("");
        println!("Please enter the input value for the angle (in radians) or ratio of sides.");
        println!("");

        let mut input_value_string = String::new();
        match io::stdin().read_line(&mut input_value_string) {
            Ok(_) => {}
            Err(_) => {
                println!("Input error. Try again.");
                println!("");
                continue;
            }
        }

        input_value = match input_value_string.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input error. Try again.");
                println!("");
                continue;
            }
        };

        input_valid = true;
    }

    return (trig_function, input_value);
}

// This checks to see if the user wishes to keep running the program.
fn ask_continue_running() -> bool {
    let mut clear_answer = false;

    while clear_answer == false {
        println!("");
        println!("Do you want to do another calculation? (y/n)");

        let mut answer_string = String::new();
        match io::stdin().read_line(&mut answer_string) {
            Ok(_) => {}
            Err(_) => {
                println!("Input error. Try again.");
                println!("");
                continue;
            }
        }

        if answer_string.trim() == "y" {
            println!("");
            return true;
        } else if answer_string.trim() == "n" {
            clear_answer = true;
            print!("");
            print!("Okay! Have a nice day!");
            print!("");

        } else {
            println!("");
            println!("Sorry, I didn't understand that answer. Please try again.");
        }
    }

    return false;
}