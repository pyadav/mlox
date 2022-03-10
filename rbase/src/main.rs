use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::{env::args, process};

fn main() {
    // Read program arguments and process
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64)
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt()
    }
}

fn run_file(path: &String) -> io::Result<()> {
    // process whole file content
    let f = File::open(path)?;

    let mut buf = Vec::new();
    let mut reader = BufReader::new(f);
    reader.read_to_end(&mut buf)?;

    run(&buf);
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();

    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            } else {
                run(&line.as_bytes())
            }
        } else {
            break;
        }
    }
}
fn run(data: &[u8]) {
    println!("{:?}", data)
}
