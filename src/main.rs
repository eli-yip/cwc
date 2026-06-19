use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use unicode_segmentation::UnicodeSegmentation;

/// A word counter that properly handles CJK and Unicode text.
///
/// Counts the given files, or reads from stdin when piped. With no arguments and
/// no piped input, recursively counts every UTF-8 text file under the current
/// directory.
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

/// How a file was selected, which governs how invalid UTF-8 and read errors
/// are handled.
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// Files named on the command line: count even invalid UTF-8 (lossily) and
    /// fail the process if any file cannot be read.
    Explicit,
    /// Files discovered by walking a directory: silently skip non-UTF-8 (binary)
    /// files and tolerate per-file read errors.
    Directory,
}

fn main() {
    let cli = Cli::parse();

    // No file arguments and input is piped: count the stdin stream.
    if cli.files.is_empty() && !io::stdin().is_terminal() {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer).unwrap_or_else(|err| {
            eprintln!("Error reading from stdin: {}", err);
            process::exit(1);
        });

        // Tolerate invalid UTF-8 by replacing bad sequences instead of failing.
        let content = String::from_utf8_lossy(&buffer);
        println!("(stdin): {} words", count_words(&content));
        return;
    }

    // Otherwise build a list of files, then read them all the same way.
    let (files, mode) = if cli.files.is_empty() {
        // No arguments: recursively count every text file under the current dir.
        let mut files = Vec::new();
        if let Err(err) = collect_files(Path::new("."), &mut files) {
            eprintln!("Error reading directory: {}", err);
            process::exit(1);
        }
        files.sort();
        (files, Mode::Directory)
    } else {
        (
            cli.files.iter().map(PathBuf::from).collect(),
            Mode::Explicit,
        )
    };

    process_files(&files, mode);
}

/// Read and count each file, then report per-file counts and a total.
fn process_files(files: &[PathBuf], mode: Mode) {
    let mut file_stats: Vec<FileStats> = Vec::new();
    let mut had_error = false;

    for path in files {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", path.display(), err);
                had_error = true;
                continue;
            }
        };

        let count = match String::from_utf8(bytes) {
            Ok(text) => count_words(&text),
            // Skip binary files when walking a directory; for an explicitly named
            // file, replace the bad sequences instead of failing.
            Err(err) => match mode {
                Mode::Directory => continue,
                Mode::Explicit => count_words(&String::from_utf8_lossy(err.as_bytes())),
            },
        };

        file_stats.push(FileStats {
            filename: path.display().to_string(),
            word_count: count,
        });
    }

    if file_stats.is_empty() && mode == Mode::Directory {
        println!("No UTF-8 text files found.");
    } else {
        report(&file_stats);
    }

    // Only an explicitly requested file makes a read failure fatal.
    if had_error && mode == Mode::Explicit {
        process::exit(1);
    }
}

/// Collect regular files under `dir`, recursing into subdirectories. Hidden
/// entries (those starting with '.', e.g. `.git`) are skipped.
fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(&entry.path(), files)?;
        } else if file_type.is_file() {
            files.push(entry.path());
        }
    }
    Ok(())
}

/// Print per-file counts, plus a total when more than one file was counted.
fn report(file_stats: &[FileStats]) {
    for stats in file_stats {
        println!("{}: {} words", stats.filename, stats.word_count);
    }

    if file_stats.len() > 1 {
        let total: usize = file_stats.iter().map(|s| s.word_count).sum();
        println!("Total: {} words", total);
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

    #[test]
    fn lossy_replacement_of_invalid_utf8_does_not_inflate_count() {
        // 0xFF is not valid UTF-8 and becomes U+FFFD, which is not a word.
        let content = String::from_utf8_lossy(b"hello \xFF world");
        assert_eq!(count_words(&content), 2);
    }
}
