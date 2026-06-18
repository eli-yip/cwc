use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process;

use clap::{CommandFactory, Parser};
use unicode_segmentation::UnicodeSegmentation;

/// A word counter that properly handles CJK and Unicode text.
///
/// Counts the given files, or reads from stdin when no file is given.
#[derive(Parser)]
#[command(name = "cwc", version, about)]
struct Cli {
    /// Files to count; reads from stdin when none are given
    files: Vec<String>,
}

struct FileStats {
    filename: String,
    word_count: usize,
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        // No files: read from stdin when piped, otherwise show help.
        if io::stdin().is_terminal() {
            Cli::command().print_help().expect("failed to print help");
            println!();
            return;
        }

        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .unwrap_or_else(|err| {
                eprintln!("Error reading from stdin: {}", err);
                process::exit(1);
            });

        println!("Word count: {}", count_words(&buffer));
        return;
    }

    // Process each file
    let mut file_stats: Vec<FileStats> = Vec::new();
    let mut total_words = 0;
    let mut had_error = false;

    for filename in &cli.files {
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
                had_error = true;
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

    // Signal failure if any file could not be read
    if had_error {
        process::exit(1);
    }
}

fn count_words(text: &str) -> usize {
    text.unicode_words().count()
}

#[cfg(test)]
mod tests {
    use super::count_words;

    #[test]
    fn counts_ascii_words() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn counts_each_cjk_character_as_a_word() {
        assert_eq!(count_words("你好世界"), 4);
    }

    #[test]
    fn counts_mixed_cjk_and_ascii() {
        assert_eq!(count_words("hello 世界"), 3);
    }

    #[test]
    fn ignores_punctuation_and_symbols() {
        assert_eq!(count_words("hello, world! 你好。"), 4);
    }

    #[test]
    fn empty_and_whitespace_only_count_zero() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   \t\n  "), 0);
    }

    #[test]
    fn collapses_repeated_whitespace() {
        assert_eq!(count_words("a   b\t\tc\nd"), 4);
    }
}
