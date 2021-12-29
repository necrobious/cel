use cel::{ choose_from_hex, error::CelError };
use clap::Parser;
use std::error::Error;
use std::str::FromStr;
use std::io::Error as IOError;
use std::io::ErrorKind;

type Choices = Vec<String>; // needed because clap gets confused by Vec<T>, thinks an option is being repeated on the CLI

#[derive(Debug,PartialEq)]
enum InputType {
    HexV1,
    //TODO: add b64 support
    //Base64,
    //Base64url,
}

impl FromStr for InputType {
    type Err = CelError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "hex" | "hex-v1" => Ok(Self::HexV1),
            //TODO: add b64 support
            //"base64" | "b64"  => Ok(Self::Base64),
            //"base64url" | "b64url" => Ok(Self::Base64url),
            _ => Err(CelError::new("unknown input type")),
        }
    }
}

fn parse_choices(src: &str) -> Result<Choices, Box<dyn Error + Send + Sync + 'static>> {
    let sv: Vec<String> = src.split('|').into_iter().map(|s| s.to_string()).collect();
    if sv.len() > 0 && sv.len() < 256 {
        Ok(sv)
    } else if sv.len() > 0 {
        Err(Box::new(CelError::new("too many choices")))
    } else {
        Err(Box::new(CelError::new("at least one choice is required")))
    }
}

#[derive(Parser, Debug)]
#[clap(about = "Choose N options from a list of available options by using a consistent hash of an input value.")]
struct Args {
    #[clap(display_order(10), short = 'c', long, takes_value = true, required = true, parse(try_from_str = parse_choices), multiple_occurrences = false, help = "Possible options to make a choice from")]
    choices: Choices,

    #[clap(display_order(11), short = 's', long, takes_value = true, required = true, help = "Number of choices to output")]
    choice_count: usize,

    #[clap(display_order(21), short = 't', long, takes_value = true, required = true, possible_values(["hex","hex-v1"]), help = "Format of the 'input' value")]
    input_type: InputType,

    #[clap(display_order(20), short = 'i', long, takes_value = true, required = true, help = "Data to convert to bytes and hash into the index of possible choices")]
    input: String,

    #[clap(display_order(90), short = 'd', long, takes_value = true, default_value="\n", help = "Delimiter to separate choices with in the output")]
    delimiter: String,

    #[clap(display_order(91), short = 'n', long, takes_value = false, help = "Do append a trailing newline to the output")]
    omit_new_line: bool,

    #[clap(display_order(99), short = 'v', long, parse(from_occurrences), multiple_occurrences = true)]
    verbose: u8,
}

fn main() -> Result<(), IOError> {
    let args = Args::parse();
    if args.verbose > 0 {
        println!("{:#?}", args);
        println!("--- ---");
    }
    if args.input_type == InputType::HexV1 {
        let choices =
            choose_from_hex(args.input, args.choices, args.choice_count)
                .map_err(|cel| IOError::new(ErrorKind::Other, cel.to_string()))?;
        print!("{}{}", choices.join(&args.delimiter), if args.omit_new_line { "" } else {"\n"} );
        return Ok(())
    }
    Err(IOError::new(ErrorKind::Other, "unsupported or unimplemented input type, PRs welcomed!"))
}
