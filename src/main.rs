use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a string to wait for, such as 'fruit'.");
        return ();
    } else if args.len() > 2 {
        println!("Only one string should be specified.");
        return ();
    }
    let characters: Vec<&str> = "a b c d e f g h i j k l m n o p q r s t u v w x y z"
        .split(" ").collect();
    let chars_len = characters.len();
    let target = String::from(&args[1]);
    let target_len = target.len();
    let mut monkey_output = String::with_capacity(target_len);
    
    let mut rng = rand::thread_rng();
    let mut num: usize;
    while monkey_output != target {
        monkey_output.clear();
        for _ in 0..target_len {
            num = rng.gen_range(0..chars_len);
            monkey_output.push_str( characters[num] );
        }
        println!("{}", monkey_output);
    }
    println!("{}={}", monkey_output, target);
}
