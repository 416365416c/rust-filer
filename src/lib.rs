use std::collections::BTreeSet; // So that the values are in sorted order
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref FROM_RE: Regex = Regex::new(r"^From [[:word:]]+@[[:word:]]+[ a-zA-Z0-9:+]*$").unwrap();
    static ref LABEL_RE: Regex = Regex::new(r"^X-Gmail-Labels: ").unwrap();
}

const INF = 99999; // For label selection algorithm

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
    return line_str[15..].trim().split(",");
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
                    for label in get_labels(line_str) {
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

//TODO: OO this, mail is a class with new and write

struct Mail {
    mut mail_buffer: String,
    pub mut target_mbox: String, //Some/None type?
}

impl Mail {
    pub fn new(&self, misc: bool) {
        if (misc) {
            target_mbox = String::from("misc");
        } else {
            target_mbox = String::from//HERE
        }
    }
    pub fn append(&self, line: &str) {
        self.mail_buffer.append(line);
    }

    pub fn write_to_mbox(&self) {
        // If mail_buffer empty or target_mbox none then skip
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
                    let mut allow_idx = INF;
                    for label in get_labels(line_str) {
                        if label in block { // If blocked, dest_str to NONE (still copying buf thou?)
                            log::fatal!("NOT YET IMPLEMENTED");
                        } else if label in allow { // Efficient?
                            log::fatal!("NOT YET IMPLEMENTED");
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
