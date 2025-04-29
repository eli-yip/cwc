use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;
use unicode_segmentation::UnicodeSegmentation;

const VERSION: &str = "1.0.2";

struct FileStats {
    filename: String,
    word_count: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for version flag
    if args.len() > 1 && args[1] == "-v" {
        println!("cwc version {}", VERSION);
        return;
    }

    // Check if we're reading from stdin (pipe)
    if !atty::is(atty::Stream::Stdin) {
        // Reading from pipe
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .unwrap_or_else(|err| {
                eprintln!("Error reading from stdin: {}", err);
                process::exit(1);
            });

        let count = count_words(&buffer);
        println!("Word count: {}", count);
        return;
    }

    // If no files provided, show usage
    if args.len() < 2 {
        println!("Usage: {} [file1] [file2] ... [fileN]", args[0]);
        println!("       or pipe text to the program");
        println!("       Use -v to display version");
        return;
    }

    // Process each file
    let mut file_stats: Vec<FileStats> = Vec::new();
    let mut total_words = 0;

    for filename in &args[1..] {
        match fs::read_to_string(filename) {
            Ok(content) => {
                let count = count_words(&content);
                total_words += count;
                file_stats.push(FileStats {
                    filename: filename.clone(),
                    word_count: count,
                });
            }
            Err(err) => {
                eprintln!("Error reading file '{}': {}", filename, err);
            }
        }
    }

    // Display results
    for stats in &file_stats {
        println!("{}: {} words", stats.filename, stats.word_count);
    }

    // If more than one file was processed, show total
    if file_stats.len() > 1 {
        println!("Total: {} words", total_words);
    }
}

fn count_words(text: &str) -> usize {
    text.unicode_words().count()
}
