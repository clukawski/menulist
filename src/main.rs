extern crate getopts;

use getopts::Options;
use std::env;
use std::io;

mod menu;
#[cfg(test)]
mod test;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FOLDER [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Output string of menu entries
    let mut output = String::new();

    // Create our options
    let mut opts = Options::new();
    // opts.optopt("o", "", "set output filename", "OUTPUT");
    opts.optopt("e", "", "specify file extension to filter", "EXTENSION");
    opts.optflag("h", "help", "print this help menu");

    // Parse options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Print help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    // Get requested file extensions
    let exts_opts = matches.opt_str("e");
    let exts = match exts_opts {
        Some(ext) => String::from(ext),
        None => String::from(""),
    };
    let exts_vec: Vec<&str> = exts.as_str().split(",").collect();

    // Grab our input path after parsing opts
    let src_path = if !matches.free.is_empty() {
        Some(String::from(matches.free[0].clone()))
    } else {
        None // Use menu::expand_path() default const
    };

    // Get menu entries output via the source path
    menu::expand_path(src_path, &exts_vec, &mut output)?;

    // Remove trailing \n from menu entries string
    println!("{}", output.trim());
    Ok(())
}
