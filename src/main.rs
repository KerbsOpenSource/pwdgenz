use arboard::Clipboard;
use clap::Parser;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;
use rand_core::OsRng;

const LETTERSET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const SPECSET: [char; 5] = ['!', '@', '#', '$', '-'];

fn get_random_number() -> String {
    let mut rng = OsRng;
    let dist = Uniform::from(0..=9);
    let random_num = dist.sample(&mut rng);
    format!("{}", random_num)
}

fn get_random_letter(uppercase: Option<bool>) -> String {
    let mut rng = OsRng;
    let letter = LETTERSET.choose(&mut rng).unwrap();
    if let Some(true) = uppercase {
        format!("{}", letter.to_uppercase())
    } else {
        format!("{}", letter)
    }
}

fn get_random_spec_char() -> String {
    let mut rng = OsRng;
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

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
struct Cli {
    /// Password length.
    #[arg(short, long, default_value_t = 16)]
    length: u32,
    /// Amount of passwords.
    #[arg(short, long, default_value_t = 1)]
    amount: u32,
    /// Save the last value to the clipboard.
    #[arg(short, long, default_value_t = false)]
    clipboard: bool,
}

fn main() {
    let cli = Cli::parse();
    let password_amount: u32 = cli.amount;
    let char_length: u32 = cli.length;
    let mut probs = vec![0.32, 0.32, 0.32, 0.04];
    for i in 0..password_amount {
        let mut password = String::from("");
        for _ in 0..char_length {
            let char_type = match char_type_choice(&probs) {
                Ok(i) => i,
                Err(e) => panic!("{}", e),
            };

            match char_type {
                0 => password.push_str(&get_random_number()),
                1 => password.push_str(&get_random_letter(Some(false))),
                2 => password.push_str(&get_random_letter(Some(true))),
                3 => password.push_str(&get_random_spec_char()),
                _ => panic!("Character type selection error!"),
            };

            // Update probabilities.
            probs[char_type] /= 2.0;
            for prob in probs.iter_mut() {
                *prob *= 1.33;
                if *prob > 1.0 {
                    *prob = 1.0;
                }
            }
        }
        print!("{}", password);
        if i != password_amount - 1 {
            println!();
        } else if cli.clipboard {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(password).unwrap();
        }
    }
}
