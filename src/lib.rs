// - Read file to object/variable
// - Extract blobs of jobs/tasks/times. Store in a vector/array/struct
// - extract the times, process maths with them. Return them to a new/same vector/array/struct
// - print the output. Possibly in future add .csv/sheets pastable output, for now just print
// to stdout.

use chrono::{NaiveTime, TimeDelta};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process;

// Wait should I have made a hashmap?

#[derive(Debug)]
struct JobEntry<'a> {
    job_date: &'a str,
    job_card: Vec<&'a str>,
    job_tasks: Vec<(i32, &'a str)>, // index of job_card, job task
    job_times: Vec<&'a str>,
    job_numerical_times: Vec<(i8, f32)>, // Same as job_times, but converted text scrawl to floating
                                         // point hours
}

impl<'a> JobEntry<'a> {
    fn populate_strings(&mut self, input_file: Vec<&'a str>) {
        // take input file or trimmed file, populate a JobEntry instance.
        //
        // Now do the magic sorting into data structures.
        // Optionally check if "date" and similar are what you think they are with regex, print
        // warnings if this is not the case.

        // I'm being naughty here. Watch me use .unwrap() in production code.
        let rx_date = Regex::new(r"^# +[0-9]{4}/[0-9]{2}/[0-9]{2}").unwrap();
        let rx_job = Regex::new(r"^# +#[a-zA-Z]{3}[0-9]{3} *.*$").unwrap();
        let rx_task = Regex::new(r"^## [a-zA-Z /]*").unwrap();
        let rx_time = Regex::new(r"^### ").unwrap(); // could be more sophisticated, but idrc

        self.job_date = input_file[0];
        // add error handling, check if this is not the first line or smthn.

        let mut job_index: i32 = -1;

        for line in input_file {
            if rx_job.is_match(line) {
                // error: This will not remove whitespace around the job card.  TODO: Fix this.
                self.job_card.push(line);
                job_index += 1;
                continue;
            } else if rx_task.is_match(line) {
                self.job_tasks.push((job_index, line));
            } else if rx_time.is_match(line) {
                self.job_times.push(line);
            }
        }
    }

    fn calculate_times(&mut self) {
        // Really this could be part of one gigantic method. But oh well.
        // - Convert times &str instances into a float.
        let rx_time = Regex::new(r"^### ").unwrap(); // could be more sophisticated, but idrc
        let time_block = Regex::new(r"([0-9]{1,2}:[0-9]{1,2}-[0-9]{1,2}:[0-9]{1,2})").unwrap();

        let mut midway_vec: Vec<(i8, &str, f32)> = vec![]; // (line index, time_str, time_elapsed_int)
        let mut line_index = 0;
        let mut dummy_num: f32 = 0.0;

        // extract time instances
        for line in &self.job_times {
            for (_, [instance]) in time_block.captures_iter(line).map(|a| a.extract()) {
                // Why the funny syntax above? Don't ask. It's a rabbit hole.
                //
                // Ok. I asked. The extract() thingy returns (entire_matched_regex, [separate, matched, regex, groups])
                // We don't care about the entire regex being returned. Just the match (really the
                // same thing in this case kinda).
                midway_vec.push((line_index, instance, dummy_num));
            }
            line_index += 1;
        }

        dbg!(&midway_vec);

        let mut time_pair: Vec<NaiveTime>;
        let mut inter_str_pair: Vec<&str>;
        let mut time_start: NaiveTime;
        let mut time_end: NaiveTime;
        let mut time_diff: TimeDelta;

        for mut entry in midway_vec {
            inter_str_pair = entry.1.split('-').collect();
            // Surely don't use unwrap here without handling. Right? Right?????
            time_start = NaiveTime::parse_from_str(inter_str_pair[0], "%H:%M").unwrap();
            time_end = NaiveTime::parse_from_str(inter_str_pair[1], "%H:%M").unwrap();
            time_diff = time_end - time_start;

            // TODO: Implement this behaviour in a nicer way please
            if time_diff.num_minutes() <= 1 {
                println!(
                    "Warning! Time difference in job entry is negative! Consider using 24hr time please"
                );
            }

            self.job_numerical_times.push((
                entry.0,
                // Cursed way of getting to 2 decimal place floats
                (time_diff.num_minutes() as f32 * 100.0 / 60.0).round() / 100.0,
            ));
        }
    }

    fn print_formatted(&self) {
        // hard lmao. Use funny colours and tables maybe
        todo!();
    }

    fn export_csv(&self) {
        // What it sounds like. Don't implement yet lol.
        todo!();
    }
}

//    # 2025/12/25
//    # #ZAP2027
//    ## General Admin
//    ### 9:30-10:45; 3:00-3:13; 11:59-2:23
//    ## R&D
//    ### 14:00-14:05
//    ## Job Admin
//    ### 17:00-17:15
//    # #TUR2017
//    ## Engineering
//    ### 14:00-1:00;
//    ## Job Admin
//    ### 17:15-17:17
//    # #PAC1978
//    ## General Admin
//    ### 17:20-18:00
//

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

    let mut vectorised_file: Vec<&str> = Vec::new();

    for header in stringified.lines() {
        vectorised_file.push(header);
    }

    dbg!(&vectorised_file);

    // Everything below this line probably should be in main()... Hmm
    // TODO: this should all be in main thanks.
    let mut day_times = JobEntry {
        job_date: "",
        job_card: Vec::new(),
        job_tasks: Vec::new(), // index of job_card, job task
        job_times: Vec::new(),
        job_numerical_times: Vec::new(), // Same as job_times, but converted text scrawl to floating
    };

    day_times.populate_strings(vectorised_file);
    dbg!(&day_times);
    day_times.calculate_times();
    // day_times.print_formatted();
}

//    struct_inst {
//        job_date => "# #ZAP2027";
//        job_card => <"ZAP2027", "TUR2017", "PAC1978">;
//        job_tasks => <(0, "General Admin"), (0, "R&D"), (0, "Job Admin"), (1, "Engineering"),
//            (1, "Job Admin"), (2, "General Admin")>;
//        job_times => <"12345", "23456", "34567", "0987", "17890", "398691">;
//    }
