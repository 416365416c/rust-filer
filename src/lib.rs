use std::collections::BTreeSet; // So that the values are in sorted order
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref FROM_RE: Regex = Regex::new(r"^From [[:word:]]+@[[:word:]]+[ a-zA-Z0-9:+]*$").unwrap();
    static ref LABEL_RE: Regex = Regex::new(r"^X-Gmail-Labels: ").unwrap();
}

const INF: u32 = 99999; // For label selection algorithm

fn get_file(input: &str) -> File {
    let input_path = Path::new(input);
    let file = match File::open(input_path) {
        Err(msg) => panic!("Failure to open {}: {}", input_path.display(), msg),
        Ok(file) => file,
    };

    log::info!("Loaded {}", input_path.display());

    return file;
}

fn get_labels(line: &str) -> Vec<&str> {
    // Skip the "X-Gmail-Labels: ", then trim
    return line[15..].trim().split(",").collect();
}

pub fn list_labels(input: &str) {
    //Read line by line, if it's a X-Gmail-Labels: line then grab one side of : and trim both sides
    //to make header_val

    let file = get_file(input);
    let mut labels = BTreeSet::new();

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line_str) => {
                if LABEL_RE.is_match(&line_str) {
                    log::trace!("Line match: {}", line_str);
                    for label in get_labels(&line_str) {
                        labels.insert(String::from(label));
                    }
                }
            },
            Err(msg) => log::error!("Failure to read line {}", msg),
        }
    }

    log::debug!("Labels {:?}", labels);
    for label in labels.iter() {
        println!("{}", label);
    }
}

struct Mail {
    mail_buffer: String,
    target_mbox: Option<String>,
}

impl Mail {
    pub fn new(misc: bool) -> Mail {
        let target_mbox: Option<String>;
        if misc {
            target_mbox = Some(String::from("misc"));
        } else {
            target_mbox = None;
        }

        Mail {
            mail_buffer: String::new(),
            target_mbox,
        }
    }

    pub fn append(&mut self, line: &str) {
        self.mail_buffer += line;
        self.mail_buffer += "\n";
    }

    pub fn write_to_mbox(&self) {
        // If mail_buffer empty or target_mbox none then skip
        if self.mail_buffer.len() == 0 {
            return;
        }

        let mbox_name = match &self.target_mbox {   
            Some(target) => target,
            None => return,
        };

        let mbox_path = Path::new(mbox_name);
        let file_attempt = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(mbox_path);

        let mut file = match file_attempt {
            Err(msg) => panic!("Failure to open {}: {}", mbox_path.display(), msg),
            Ok(file) => file,
        };
        
        let write_attempt = file.write_all(self.mail_buffer.as_bytes());
        match write_attempt {
            Err(msg) => log::error!("Write error for {}: {}", mbox_path.display(), msg),
            Ok(_any) => (),
        }
    }
}

pub fn file_mails(input: &str, allow: Vec<&str>, block: Vec<&str>, misc: bool) {
    let file = get_file(input);
    let mut mail = Mail::new(misc);

    for line in BufReader::new(file).lines() {
        match line {
            Ok(line_str) => {
                if FROM_RE.is_match(&line_str) {
                    mail.write_to_mbox();
                    mail = Mail::new(misc);
                } else if LABEL_RE.is_match(&line_str) {
                    let mut allow_idx: u32 = INF;
                    for label in get_labels(&line_str) {
                        if block.contains(&label) {
                            mail.target_mbox = None;
                            break;
                        } else if allow.contains(&label) { // Efficient?
                            //TODO: Label priority
                            mail.target_mbox = Some(String::from(label));
                        }
                    }
                }
                mail.append(&line_str)
            },
            Err(msg) => log::error!("Failure to read line {}", msg),
        }
    }

    mail.write_to_mbox();
}
