use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::env;
use std::io;
use std::path::Path;
use regex::Regex;
use colored::*;
use chrono::prelude::*;

fn check() {}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return
            Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "specify file to watch in arguments! Example: file_reader C:\\tools\\etamps\\swadlwrapper.log"));
    }

    let file_name = args.get(1).ok_or(io::Error::new(io::ErrorKind::InvalidInput, "failed to get name of file specified")).unwrap();
    let path = Path::new(&file_name);
    let file = File::open(path)?;
    let mut line_iterator = BufReader::new(file).lines().into_iter();

    let pattern = Regex::new("[Ss][Ee][Nn][Dd][Ii][Nn][Gg]\\s+\\D*(\\d+)\\s+\\D*[Ss][Ee][Nn][Tt]\\s+\\D*(\\d+)").unwrap();

    println!("This console tool reads logs file (usually named \"swadlwrapper.log\" and interprets output to display upload progress.");
    println!("When tool sees line with 'Sending' and 'Send' words with numbers after EACH of them, it interprets this line as a helthy output.");
    println!("which indicates that upload is progressing as intended.");
    println!("If such pattern is not found - tool warns user with message: \"This log line indicates no data transfer in this moment!\"");

    loop {
        loop {
            let line = line_iterator.next();
            let line = if line.is_none() {
                break;
            } else {
                let res = line.unwrap();
                if res.is_err() {
                    continue;
                } else {
                    res.unwrap()
                }
            };

            let captures = pattern.captures(&line);

            let captures = if captures.is_none() {
                println!("{} Logs from file \"{}\": {}", "This log line indicates no data transfer in this moment!".red(), file_name, line.yellow());

                continue;
            } else {
                captures.unwrap()
            };

            if captures.len() < 3 {
                continue;
            }

            let pack_size = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let sent_bytes_number = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let pack_size_as_str = format!("{}", captures.get(1).unwrap().as_str().parse::<u64>().unwrap()).blue();
            let sent_bytes_number_as_str = format!("{}", captures.get(2).unwrap().as_str().parse::<u64>().unwrap()).blue();
            let percentage = format!("{:.1}", ((sent_bytes_number as f32 / pack_size as f32) * 100 as f32)).green();

            let current_time = Local::now().format("%H:%M:%S").to_string().cyan();

            println!("{} :: Uploaded {:>10} / {}[kB] == {:>10}%", current_time, sent_bytes_number_as_str, pack_size_as_str, percentage);
        }

        thread::sleep(Duration::from_millis(200));
    }
}
