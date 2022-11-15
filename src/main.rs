use rand::prelude::*;
use std::io::{ BufRead, BufReader };

struct TypingBias {
    bias_map: [usize; 256],
    total: usize,
    has_value: bool
}

impl TypingBias {
    fn new() -> TypingBias {
        TypingBias {
            bias_map: [0; 256],
            total: 0,
            has_value: false
        }
    }

    fn add_bias(&mut self, character: u8, amt: usize) {
        self.has_value = true;
        self.bias_map[character as usize] += amt; 
        self.total += amt;
    }

    fn generate_test(&self, rng: &mut ThreadRng, char_amt: usize) -> String {
        // if there are no bias's, then just return
        if self.has_value == false {
            return String::from("");
        }
        
        let mut s = String::new();

        // find the first non-zero value
        // this way that if we roll a number before
        // the first char, it will not activate it
        let mut start = 0;
        for i in 0..256 {
            if self.bias_map[i] != 0 {
                start = i;
                break;
            }
        }

        for _ in 0..char_amt {
            let num = rng.gen_range(0..self.total);
            let mut sum = 0;
            let mut index = start;

            // find the last value that is lesser than the rolled rng
            // because we add up al lthe bias's and the generated range is
            // from 0..total, they all have the same probability
            while sum < num {
                sum += self.bias_map[index];
                index += 1;
            }

            // because \0 is invalid, -1 is ok to subtract
            // and will not overflow
            s.push((index - 1) as u8 as char);
        }
        s
    }
}

fn load_bias_from_stream(mut input: impl BufRead, bias: &mut TypingBias) -> std::io::Result<()> {
    let mut buffer = String::new();

    let mut amt_read = 1;
    let mut line = 0;
    // go through and read every line from the file
    while amt_read != 0 {
        amt_read = input.read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            break;
        }
        // the first character is the mapped value,
        // the second value is the rest, after the delimiting character
        // that gives us the format 1:2 or 1 2 to represend 1 2 times
        let character = &buffer[0..1];
        let rest = buffer[2..].trim();

        let character = character.as_bytes()[0];
        let amount = match rest.parse::<usize>() {
            Ok(amt) => amt,
            Err(_) => print_format_message(line)
        };
        
        bias.add_bias(character, amount);

        line += 1;
        buffer.clear();
    }

    Ok(())
}

fn print_format_message(line: u64) -> ! {
    const FILE_FORMAT_MESSAGE: &'static str = "bias should be new line seperated with the format {char}:{bias}";
    eprintln!("incorrect format on line: {}\n{}", line, FILE_FORMAT_MESSAGE);
    std::process::exit(-1);
}

fn print_usage_message() -> ! {
    const USAGE_MESSAGE: &'static str = "provide the amount of characters to generate";
    eprintln!("{}", USAGE_MESSAGE);
    std::process::exit(-2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut bias = TypingBias::new();
    let mut rng = rand::thread_rng();

    let amt = match args.get(1) {
        Some(s) => match s.parse::<usize>() {
            Ok(amt) => amt,
            Err(_) => print_usage_message()
        }
        None => print_usage_message()
    };

    // read from io, and parse the format into a bias
    let input = std::io::stdin();
    let buf_input = BufReader::new(input);
    load_bias_from_stream(buf_input, &mut bias).unwrap();
    
    // generate the string and give it back
    let output = bias.generate_test(&mut rng, amt);
    println!("{}", output);
}
