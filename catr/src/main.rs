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
            Ok(f) => {
                let mut line_no = 1;
                for line in f.lines() {
                    if args.number_lines {
                        println!("{line_no:>6}\t{}", line.unwrap());
                        line_no += 1;
                    } else if args.number_nonblank_lines {
                        let line_actual = line.unwrap();
                        if line_actual.is_empty() {
                            println!("");
                        } else {
                            println!("{line_no:>6}\t{}", line_actual);
                            line_no += 1;
                        }
                    } else {
                        println!("{}", line.unwrap());
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