mod file_stats;
mod file_stats_error;

use std::path::PathBuf;
use clap::Parser;
use crate::file_stats::FileStats;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli{
    #[arg(short = 'm', default_value_t = true, long, help= "Count the number of characters in a file.")]
    characters: bool,
    #[arg(short = 'c', default_value_t = true, long, help= "Count the number of bytes in a file.")]
    bytes: bool,
    #[arg(short, long, default_value_t = true, help= "Count the number of words in a file.")]
    words: bool,
    #[arg(short, long, default_value_t = true, help= "Count the number of lines in a file.")]
    lines: bool,
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>
}

fn main() {
    let args = Cli::parse();

    let mut file_stats = match args.file {
        None => {
            FileStats::new()
        }
        Some(path) => {
            match FileStats::from(path){
                Ok(file) => {file}
                Err(error) => {
                    println!("{}", error.to_string());
                    return
                }}
        }
    };

    match file_stats.process(){
        Ok(_) => {
            //Output is :
            // SPACE SPACE LINES WORDS CHARACTERS BYTES FILENAME
            let mut answer: String = "  ".to_string();
            if args.lines {
                answer = format!("{} {}", answer, file_stats.get_line_count());
            }

            if args.words {
                answer = format!("{} {}", answer, file_stats.get_words_count());
            }

            if args.characters {
                answer = format!("{} {}", answer, file_stats.get_character_count());
            }

            if args.bytes {
                answer = format!("{} {}", answer, file_stats.get_bytes_count());
            }

            answer = format!("{} {}", answer, file_stats.get_file_name());
            println!("{}", answer);
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}
