use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::env;
use std::io;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    println!("current dir: {}", env::current_dir().unwrap().display());

    if args.len() < 2 {
        return Result::Err(io::Error::new(io::ErrorKind::InvalidInput, "specify file to watch in invocation of this program!"));
    }

    let file_name = args.get(1).ok_or(io::Error::new(io::ErrorKind::InvalidInput, "failed to get name of file specified")).unwrap();

    let path = Path::new(&file_name);

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut li = lines.into_iter();

    loop {
        // let reader2 = reader.copy();

        loop {
            let obj = li.next();
            if obj.is_none() {
                break;
            }

            println!("{}", obj.unwrap().unwrap());
        }

        thread::sleep(Duration::from_secs(1));
    }
}
