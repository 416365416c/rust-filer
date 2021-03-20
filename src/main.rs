use clap::{load_yaml,App};
use rust_filer::*;

fn main() {
    env_logger::init();

    //clap macro seems already at compile time
    //let yaml = include_bytes!("cli.yaml"); // &'static [u8; N] type
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    
    let input = matches.value_of("input").unwrap();
    let allow_list: Vec<&str> = matches.value_of("allow_list").unwrap().split(",").collect();
    let block_list: Vec<&str> = matches.value_of("block_list").unwrap().split(",").collect();

    let misc_enabled = matches.is_present("misc");
    let list_mode = matches.is_present("labels");
    
    log::trace!("input {}", input);
    log::trace!("m? {} l? {} a: {:?} b: {:?}", misc_enabled, list_mode, allow_list, block_list);
    if list_mode {
        list_labels(input);
    } else {
        file_mails(input, allow_list, block_list, misc_enabled);
    }
}
