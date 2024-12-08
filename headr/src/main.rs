use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value("-"))]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'), 
        long,
        default_value("10"),
        value_name = "LINES", 
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,
    
    /// Number of bytes
    #[arg( 
        short('c'), 
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
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

// fn check_byte_optional(optional: Option<>) {
//     match optional {
//         Some(p) => p,
//         None => nil,
//     }
// }

fn run(args: Args) -> Result<()> {
    dbg!(&args);

    let show_header = args.files.len() > 1;
    let via_bytes: bool = args.bytes.is_some();

    // let option_byte = check_byte_optional(args.bytes);
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                if show_header {
                    println!("\n==> {filename} <==");
                }
                if via_bytes {
                    const byte_limit: usize = 4;
                    let mut buffer = [0; byte_limit];
                    file.read_exact(&mut buffer)?;
                    println!("{:#?}", buffer);
                } else {
                    let limit: usize = args.lines.try_into().unwrap();
                    for (line_num, line_result) in file.lines().enumerate() {
                        let line = line_result?;
                        if limit <= line_num {
                            break;
                        }
                        println!("{line}");
                    }
                }
            },        
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
