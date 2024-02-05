use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;
use rand_core::OsRng;
use std::env;

const LETTERSET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const SPECSET: [char; 5] = ['!', '@', '#', '$', '-'];

fn number_char() -> String {
    let mut rng = OsRng;
    let dist = Uniform::from(0..=9);
    let random_num = dist.sample(&mut rng);
    format!("{}", random_num)
}

fn letter_char(uppercase: Option<bool>) -> String {
    let mut rng = rand::thread_rng();
    let letter = LETTERSET.choose(&mut rng).unwrap();
    if let Some(true) = uppercase {
        format!("{}", letter.to_uppercase())
    } else {
        format!("{}", letter)
    }
}

fn spec_char() -> String {
    let mut rng = rand::thread_rng();
    let letter = SPECSET.choose(&mut rng).unwrap();
    format!("{}", letter)
}

fn char_type_choice(probs: &[f32]) -> Result<usize, &'static str> {
    let mut rng = OsRng;
    let total_prob = probs.iter().sum::<f32>();
    let mut random_num: f32 = rng.gen_range(0.0..total_prob);
    for (i, &prob) in probs.iter().enumerate() {
        if random_num < prob {
            return Ok(i);
        }
        random_num -= prob;
    }
    Err("Invalid probability distribution")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut char_count: u32 = 16;
    let mut password_count: u32 = 1;
    if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(num) => char_count = num,
            Err(e) => eprintln!("Error: {}", e),
        }
        if args.len() > 2 {
            match args[2].parse::<u32>() {
                Ok(num) => password_count = num,
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
    let mut probs = vec![0.32, 0.32, 0.32, 0.04];
    for i in 0..password_count {
        let mut password = String::from("");
        for _ in 0..char_count {
            let char_type = match char_type_choice(&probs) {
                Ok(i) => i,
                Err(e) => panic!("{}", e),
            };

            match char_type {
                0 => password.push_str(&number_char()),
                1 => password.push_str(&letter_char(Some(false))),
                2 => password.push_str(&letter_char(Some(true))),
                3 => password.push_str(&spec_char()),
                _ => password.push_str(&spec_char()),
            };

            // Update probabilities
            probs[char_type] /= 2.0;
            for prob in probs.iter_mut() {
                *prob *= 1.33;
                if *prob > 1.0 {
                    *prob = 1.0;
                }
            }
        }
        print!("{}", password);
        if i != password_count - 1 {
            println!();
        }
    }
}