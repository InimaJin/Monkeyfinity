use std::env;
use rand::Rng;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a target string to wait for, such as 'fruit'.");
        println!("Type -h for help.");
        return ();
    }

    let characters: Vec<&str> = "a b c d e f g h i j k l m n o p q r s t u v w x y z"
        .split(" ").collect();
     
    let instructions = Instructions::parse_args(args)
        .unwrap_or_else(|| {
            println!("{}", help_text());
            process::exit(0);
        });
    for ch in instructions.target.chars() {
        if !ch.is_ascii_alphabetic() {
            println!("Invalid target string. Should be English letters only.");
            process::exit(1);
        }
    }

    if instructions.verbose_level == 0 {
        println!("{}", monkey_image());
    }

    let mut monkey_output = String::with_capacity(instructions.target.len());
   
    let mut rng = rand::thread_rng();
    let mut num: usize;
    let mut i = 0; // Counting iterations
    while monkey_output != instructions.target {
        i += 1;
        monkey_output.clear();
        for _ in 0..instructions.target.len() {
            num = rng.gen_range(0..characters.len());
            monkey_output.push_str( characters[num] );
        }
        
        if instructions.verbose_level > 0 {
            if instructions.verbose_level == 1 { println!("{}", monkey_output) }
            else { println!("{} - {}", i, monkey_output) }
        }
    }
    println!("{}={}\nAfter {} failed words", monkey_output, instructions.target, i);
}

// Struct holding info for program based on arguments
struct Instructions {
    target: String,
    verbose_level: u32,
}

impl Instructions {
    // Parses command line arguments and returns an instance of Instructions based on the
    // given arguments. Wrapped in an Option, it is None if the user specified '-h'.
    fn parse_args(args: Vec<String>) -> Option<Self> {
        let mut instance = Self {
            target: String::new(),
            verbose_level: 0
        };
        for arg in &args[1..] {
            match &arg[..] {
                "-h" => return None,
                "-v" => instance.verbose_level = 1,
                "-vv" => instance.verbose_level = 2,
                _ => {
                    if instance.target.len() == 0 {
                        instance.target = arg.to_lowercase()
                    } else {
                        println!("Invalid arguments: Only one input string is permitted.");
                        process::exit(1);
                    }
                }
            }
        }
    
        Some(instance)
    }
}

fn help_text() -> &'static str {
"
MONKEYFINITY - Infinite monkey theorem in Rust!

Usage:
    ./monkeyfinity TARGET_STRING [OPTIONS]

TARGET_STRING...
    - must consist of English letters only (a-z).
    - will automatically be converted to lowercase.

Only one string is allowed as the target.

AVAILABLE OPTIONS:
    -h      Prints this help menu.
    -v      Verbose output (Warning: Might severely impact performance.)
    -vv     Like -v, but also displays iteration count.
"
}

fn monkey_image() -> &'static str {
r#"
                        .="=.
                      _/.-.-.\_     _
                     ( ( o o ) )    ))
                      |/  "  \|    //
      .-------.        \'---'/    //
     _|~~ ~~  |_       /`"""`\\  ((
   =(_|_______|_)=    / /_,_\ \\  \\
     |:::::::::|      \_\\_'__/ \  ))
     |:::::::[]|       /`  /`~\  |//
     |o=======.|      /   /    \  /
     `"""""""""`  ,--`,--'\/\    /
                   '-- "--'  '--'
"#
}
