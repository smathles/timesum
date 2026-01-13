// - Read file to object/variable
// - Extract blobs of jobs/tasks/times. Store in a vector/array/struct
// - extract the times, process maths with them. Return them to a new/same vector/array/struct
// - print the output. Possibly in future add .csv/sheets pastable output, for now just print
// to stdout.

use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process;

pub fn process_file(path: &PathBuf) {
    dbg!(path);
    let file_as_string = fs::read_to_string(path);
    let stringified = match file_as_string {
        Ok(file) => file,
        Err(error_message) => {
            println!("Ok, thing broke at the \"process_file()\" function.");
            dbg!(error_message);
            process::exit(1);
        }
    };

    // I'm being naughty here. Watch me use .unwrap() in production code.
    let rx_date = Regex::new(r"^# +[0-9]{4}/[0-9]{2}/[0-9]{2}").unwrap();
    let rx_job = Regex::new(r"^# +#[a-zA-Z]{3}[0-9]{3} *.*$").unwrap();
    let rx_task = Regex::new(r"^## [a-zA-Z /]*").unwrap();
    let rx_time = Regex::new(r"^### ").unwrap(); // could be more sophisticated, but idrc

    let mut trimmed_file: Vec<&str> = Vec::new();

    for header in stringified.lines() {
        if (rx_date).is_match(header)
            || (rx_job).is_match(header)
            || (rx_task).is_match(header)
            || (rx_time).is_match(header)
        {
            let line = header;
            trimmed_file.push(line);
        }
    }

    dbg!(trimmed_file);

    // Now do the magic sorting into data structures.
    // Optionally check if "date" and similar are what you think they are with regex, print
    // warnings if this is not the case.

    // let date = get_date(&stringified);
    // add struct stuff, figure out how to store time data.
    // Job
    // |   \
    // |    \
    // |     \
    // |      \
    // |       \
    // Type_vec  hours_vec
    //
    // or type1/hours1 vec, etc. Or structs etc.

    // println!("Date: {}", date);
}
