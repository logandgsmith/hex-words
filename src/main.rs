// Reimplementing hex-words.py script in Rust
// This script attempts to find English words that can be expressed with hexadecimal numbers.
// I initially wrote this since I met this guy who went by 0xb7ade (hexblade) and wanted to see what
// other names you could make like this

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{PathBuf, Path};

use clap::Parser;

// Help info
#[derive(Parser, Debug)]
#[command(
    name = "hex-words",
    version,
    about = "Finds all the words in a given wordlist that can be created with hexadecimal numbers",
    long_about = None
)]

// CLI Arguments
struct Args {
    // positional argument
    #[arg(
        help = "Path to the wordlist. NOTE: We\'re looking for a TXT file."
    )]
    path: PathBuf,

    #[arg(
        short,
        long,
        help = "Path to output the found words."
    )]
    output: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "If provided, will append the hex translation"
    )]
    translate: bool
}

// Reads in a newline delimited file and creates a Vec from the values
fn read_words_from_file<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    // Convert wordlist into Vec
    let mut words = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let word = line.trim();
        if !word.is_empty() {
            words.push(word.to_string());
        }
    }
    Ok(words)
}

// Finds words that can be translated in the wordlist. Optionally, translates
// the word into hexadecimal numbers.
fn find_words(wordlist: Vec<String>, translate: bool) -> io::Result<Vec<String>> {
    let valid_letters: HashSet<char> = "abcdefgilostz".chars().collect();
    let mut valid_words = Vec::new();
    for word in wordlist {
        // Check if all letters in word are translatable
        if word.chars().collect::<HashSet<char>>().is_subset(&valid_letters) {
            if translate {
                let translated = translate_to_hex(&word);
                valid_words.push(format!("{}:{}", word, translated));
            } else {
                valid_words.push(word);
            }
        }
    }
    Ok(valid_words)
}

// Translates a word into hexadecimal numbers. Returns '?' if can't translate a char
fn translate_to_hex(word: &str) -> String {
    let letter_to_hex = HashMap::from([
        ('a', 'A'),
        ('b', 'B'),
        ('c', 'C'),
        ('d', 'D'),
        ('e', 'E'),
        ('f', 'F'),
        ('g', '6'),
        ('i', '1'),
        ('l', '1'),
        ('o', '0'),
        ('s', '5'),
        ('t', '7'),
        ('z', '2')
    ]);

    // Convert the word by letter
    let hex_string: String = word
        .chars()
        .map(|c| letter_to_hex.get(&c).unwrap_or(&'?'))
        .collect();

    format!("0x{}", hex_string)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path: &PathBuf = &args.path;
    let words = read_words_from_file(path)?;

    // Find words that can be expressed with hexadecimal numbers
    let mut valid_words = find_words(words.clone(), args.translate)?;
    valid_words.sort();

    // Debugging
    // for word in valid_words {
    //     println!("{}", word);
    // }

    // Write the wordlist to a file
    if let Some(output_file) = args.output {
        let mut file = File::create(&output_file)?;
        for word in &valid_words {
            match writeln!(file, "{}", word) {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to write results!");
                    break;
                }
            };
        }
        println!("Successfully wrote results to {}!\n", output_file.display());
    }

    // Basic Stats
    let total_words = words.len();
    let total_valid = valid_words.len();

    println!("== STATS ==");
    println!("Total Words in Wordlist: {}", total_words);
    println!("Valid Words: {}", total_valid);
    println!("Percentage of wordlist expressable as Hexadecimals: ~{:.4}%", total_valid as f64 / total_words as f64 * 100.0);
    println!();

    Ok(()) // Success
}
