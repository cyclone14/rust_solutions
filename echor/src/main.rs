use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("echor")
    .version("0.1.0")
    .author("Jonn Price <jprice1947@comcast.net")
    .about("Rust version of 'echo'")
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .action(ArgAction::SetTrue)
        .help("Do not print newline"),
    )
    .get_matches();

    println!("{:#?}", matches);
}
