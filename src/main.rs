use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::env;
use std::io;
use std::path::Path;
use regex::Regex;

fn check() {

}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return
            Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "specify file to watch in invocation of this program!"));
    }

    let file_name = args.get(1).ok_or(io::Error::new(io::ErrorKind::InvalidInput, "failed to get name of file specified")).unwrap();
    let path = Path::new(&file_name);
    let file = File::open(path)?;
    let mut line_iterator = BufReader::new(file).lines().into_iter();

    let pattern = Regex::new("[Ss][Ee][Nn][Dd][Ii][Nn][Gg]\\s+\\D*(\\d+)\\s+\\D*[Ss][Ee][Nn][Tt]\\s+\\D*(\\d+)").unwrap();

    loop {
        loop {

            let line = line_iterator.next();
            if line.is_none() {
                break;
            }

            let line_string = line.unwrap().unwrap();
            let captures = pattern.captures(&line_string);

            if captures.is_none(){
                break;
            }

            let matches = captures.unwrap();

            for (index, matched) in matches.iter().enumerate() {
                println!("{}: {}", index, matched.unwrap().as_str());
            }
        }

        thread::sleep(Duration::from_millis(200));
    }
}
