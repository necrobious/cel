use error::CelError;

pub fn is_hex_digit(chr: u8) -> bool {
    (chr >= 0x30 && chr <= 0x39) || (chr >= 0x41 && chr <= 0x46) || (chr >= 0x61 && chr <= 0x66)
}

pub fn choose_from_hex (input: String, options: Vec<String>, num_of_choices: usize) -> Result<Vec<String>, CelError> {
    if input.len() < 1 {
        return Err(CelError::new("empty hex input"));
    }
    if num_of_choices < 1 {
        return Err(CelError::new("must choose at least one choice"))
    }
    if options.len() < 1 {
        return Err(CelError::new("need at least one choice to choose from"))
    }
    if options.len() > 255 {
        return Err(CelError::new("too many choices to choose from"))
    }
    if input.len().checked_rem(2) != Some(0) {
        return Err(CelError::new("invalid hex input"));
    }
    if !input.as_bytes().iter().all(|chr| is_hex_digit(*chr)) {
        return Err(CelError::new("invalid hex input"));
    }

    // convert input hex string to bytes, reduce each byte by xor'ing it to the accumulator
    let mut acc = 0;
    let mut i = 0;
    while i < input.len() {
        let b = u8::from_str_radix(&input[i..(i+2)], 16);
        if b.is_err() { return Err(CelError::new("invalid hex value")) }
        acc ^= b.unwrap();
        i += 2;
    }
    // let options_count = options.len() as u8;
    let start_at = (acc % (options.len() as u8)) as usize;
    let mut choices = Vec::with_capacity(num_of_choices);
    for i in 0..num_of_choices {
        choices.push(options[(start_at + i) % options.len()].clone())
    }
    Ok(choices)
}

pub mod error;