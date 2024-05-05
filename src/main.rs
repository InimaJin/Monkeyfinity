use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a string to wait for, such as 'fruit'.");
        return ();
    } else if args.len() > 2 {
        println!("Only one string should be specified.");
        return ();
    }
    let target = String::from(&args[1]);
    let mut monkey_output = String::with_capacity(target.len());
}
