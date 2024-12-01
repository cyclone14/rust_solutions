use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value("-"))]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'), 
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(
        short('b'), 
        long("number-nonblank")
        // conflicts_with("number_lines")
    )]
    number_nonblank_lines: bool,
} 

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        // println!("{filename}");
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut prev_num = 0; // used to count the non-blank lines
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:>6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:>6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            },
        }    
    }    
    Ok(())
}    

fn main() {
    // let args = Args::parse();
    // dbg!(args);
    // println!("{args:#?}");
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

}