// - Read file to object/variable
// - Extract blobs of jobs/tasks/times. Store in a vector/array/struct
// - extract the times, process maths with them. Return them to a new/same vector/array/struct
// - print the output. Possibly in future add .csv/sheets pastable output, for now just print
// to stdout.

use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process;

// struct JobEntry<'a> {
//     // job_date: &'a str,
//     job_card: Vec<&'a str>,
//     job_tasks: Vec<&'a str>,
//     job_times: Vec<&'a str>,
// }

// Wait should I have made a hashmap?

struct JobEntry<'a> {
    job_date: &'a str,
    job_card: Vec<&'a str>,
    job_tasks: Vec<(u8, &'a str)>, // index of job_card, job task
    job_times: Vec<&'a str>,
    job_numerical_times: Vec<f32>, // Same as job_times, but converted text scrawl to floating
                                   // point hours
}

impl<'a> JobEntry<'a> {
    fn populate_strings(&self, input_file: Vec<&'a str>) {
        // take input file or trimmed file, populate a JobEntry instance.
        todo!();
    }

    fn calculate_times(&self) {
        // Really this could be part of one gigantic method. But oh well.
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
//
//
//    struct_inst {
//        job_date => "# #ZAP2027";
//        job_card => <"ZAP2027", "TUR2017", "PAC1978">;
//        job_tasks => <(0, "General Admin"), (0, "R&D"), (0, "Job Admin"), (1, "Engineering"),
//            (1, "Job Admin"), (2, "General Admin")>;
//        job_times => <"12345", "23456", "34567", "0987", "17890", "398691">;
//    }
//
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

    // I'm being naughty here. Watch me use .unwrap() in production code.
    let rx_date = Regex::new(r"^# +[0-9]{4}/[0-9]{2}/[0-9]{2}").unwrap();
    let rx_job = Regex::new(r"^# +#[a-zA-Z]{3}[0-9]{3} *.*$").unwrap();
    let rx_task = Regex::new(r"^## [a-zA-Z /]*").unwrap();
    let rx_time = Regex::new(r"^### ").unwrap(); // could be more sophisticated, but idrc

    let mut trimmed_file: Vec<&str> = Vec::new();

    // Realistically I could have done this with just "^#". But anyway. Change this later once I've
    // been a good boy and written tests (which I will do right?)
    // ...right?
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

    // BUG: The logic below "make new instance of data structure every loop" will not work.
    //
    // Solution: Store *all* of this in a big datastructure. Even if it's a bit gross. You can add
    // the logic to tease the required info out later.
    //
    // Desired logic:
    // - Iterate through file
    //   - Store date
    // - continue iteration
    // - find first/next job card. Save to new data structure (Vec?)
    // - iterate, adding all instances of *task* or *time* to the data structure --until-- a new job card is found.
    //          - yes, I know there are ways of bricking this. shut up.
    // - Stop adding entries to data structure. Make new instace of data structure and repeat above
    // lines.
    // - Continue until reach end of file.
    //
    //
    //
    //
    //
    // - Result: several data structures, holding:
    //      - (opt) date
    //      - job card
    //      - array/vec/tuple/something of all the job tasks
    //      - array/vec/tuple/something of all the job times
    //
    // - Convert times &str instances into a float.
    //
    // - Print all data to a nice colourful window with tables and whatnot.
    //
    // - Add a TODO to make a .CSV export flag. Or worse formats.
}
