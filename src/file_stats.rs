use std::fs::File;
use std::io;
use std::io::{BufRead, ErrorKind};
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;
use crate::file_stats_error::FileStatsError;

pub struct FileStats{
    file: Option<File>,
    bytes: usize,
    characters: usize,
    words: usize,
    lines: usize,
    filename: String
}

impl FileStats {
    pub fn new() -> FileStats {
        FileStats {
            words: 0,
            lines: 0,
            filename: "".to_string(),
            bytes: 0,
            characters: 0,
            file: None
        }
    }

    pub fn from(path: PathBuf) -> Result<FileStats, FileStatsError>{
        let filename = match path.file_name() {
            None => {""}
            Some(path) => {match path.to_str(){
                None => {""}
                Some(s) => {s}
            }}
        };
        match FileStats::open_file(&path){
            Ok(f) => {
                Ok(FileStats{
                    file: Some(f),
                    bytes: 0,
                    lines: 0,
                    words: 0,
                    characters: 0,
                    filename: filename.to_string()
                })
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn process(&mut self) -> Result<(), FileStatsError> {
        match self.file{
            None => {self.process_stdin()}
            Some(_) => {self.process_file()}
        }
    }

    fn process_stdin(&mut self) -> Result<(), FileStatsError>{
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(s) => {
                    self.bytes += FileStats::count_bytes(&s);
                    self.characters +=  FileStats::count_characters(&s);
                    self.words += FileStats::count_words(&s);
                    self.lines += 1;
                }
                Err(err) => {
                    return match err.kind(){
                        ErrorKind::NotFound => {
                            Err(FileStatsError::NotFound)
                        }
                        ErrorKind::PermissionDenied => {
                            Err(FileStatsError::PermissionDenied)
                        }
                        _ => {
                            Err(FileStatsError::Other {msg: err.to_string()})
                        }
                    };
                }
            }
        }
        Ok(())
    }

    fn process_file(&mut self) -> Result<(), FileStatsError> {
        let binding = match &self.file{
            None => {
                return Err(FileStatsError::Other {msg: "No file pass in parameter".to_string()})
            }
            Some(f) => {f}
        };
        let mut buf = io::BufReader::new(binding);
        let mut s = String::new();
        loop {
            match buf.read_line(&mut s) {
                Ok(read_bytes) => {
                    if read_bytes == 0 {
                        //We ended the stream
                        break;
                    }

                    self.bytes += FileStats::count_bytes(&s);
                    self.characters +=  FileStats::count_characters(&s);
                    self.words += FileStats::count_words(&s);
                    self.lines += 1;

                    //Empty the string buffer that we read into
                    s.clear();
                }
                Err(err) => {
                    return match err.kind(){
                        ErrorKind::NotFound => {
                            Err(FileStatsError::NotFound)
                        }
                        ErrorKind::PermissionDenied => {
                            Err(FileStatsError::PermissionDenied)
                        }
                        _ => {
                            Err(FileStatsError::Other {msg: err.to_string()})
                        }
                    };
                }
            }
        }
        Ok(())
    }

    pub fn get_bytes_count(&self)-> usize{
        self.bytes
    }

    pub fn get_character_count(&self)-> usize{
        self.characters
    }

    pub fn get_line_count(&self) -> usize {
        self.lines
    }

    pub fn get_words_count(&self) -> usize {
        self.words
    }

    pub fn get_file_name(&self) -> String {
        self.filename.clone()
    }

    fn open_file(path: &PathBuf) -> Result<File, FileStatsError>{
        match File::open(&path) {
            Ok(file) => {Ok(file)}
            Err(err) => {
                match err.kind(){
                    ErrorKind::NotFound => {
                        Err(FileStatsError::NotFound)
                    }
                    ErrorKind::PermissionDenied => {
                        Err(FileStatsError::PermissionDenied)
                    }
                    _ => {
                        Err(FileStatsError::Other {msg: err.to_string()})
                    }
                }
            }
        }
    }

    fn count_bytes(buf: &String) -> usize{
        buf.len()
    }

    fn count_characters(buf: &String) -> usize{
        buf.graphemes(true).count()
    }

    fn count_words(buf: &String) -> usize{
        words_count::count(&buf).words
    }
}