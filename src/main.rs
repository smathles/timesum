use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;
use timesum::process_file;

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "A basic utility to convert daily logs into timesheet outputs"
)]
struct Cli {
    // NOTE: PathBuf is like a String but for file system paths that work cross-platform.
    // TODO: Do I really actually need a struct with only one command? ye, might add some stuff later.
    path: Option<PathBuf>,
    // Other options, idk.
}

// impl Cli {
//     fn changeme() {
//         todo!();
//     }
// }

fn main() {
    {
        // // env::args() returns an iterator of passed commands from the command line. First element is
        // // generally the executable path.
        // //
        // // .nth() returns the 'n'th element of an iterator.
        // //
        // // .expect() returns the contained [`Some`] value of an Option() object, consuming the `self` value.
        // // Custom panic message provided (bad practice in production lmao).
        //
        // // Note that .nth(0) is the executable file path. We don't really care about this.
        // let pattern = env::args().nth(1).expect("No pattern given");
        // let path = env::args().nth(2).expect("No pattern given");
        //
        // println!("pattern: {:?}, path: {:?}", pattern, path);
        //
        // let args = Cli {
        //     path: std::path::PathBuf::from(path),
        // };
    }
    // alternative to env::args::collect() with additional functionality.
    let args = Cli::parse();
    // dbg!(&args);

    // Check filepath was provided.
    let filepath = match &args.path {
        Some(path_passed) => path_passed,
        None => {
            println!("No filepath specified idiot >:(");
            process::exit(1);
        }
    };

    // Check file exists
    match fs::exists(filepath) {
        Ok(file_exists) => {
            if file_exists != true {
                println!("File does not exist!");
                process::exit(1);
            }
        }
        Err(e) => {
            println!(
                "Literally how did you do that? What magic and tomfoolery were you trying to accomplish? \n Did you inject a weird prompt? Bad user. BAD user. Stop."
            );
            eprintln!("Ok, so the error is: {e}");
            // don't need to return a boolean here, the program is ded!
            process::exit(1);
        }
    };

    // Process file
    // Should break this up, but hey here we are.
    process_file(filepath);
}
