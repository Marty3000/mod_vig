//! simple cli-Interface for the modernized _vigenere lib
//!
//! encrypt and decrypt any files with a vigenere cipher based on bytes ( not on the alphabet )
#![forbid(unsafe_code)]

use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// print verbose messages
    #[clap(short, long)]
    verbose: bool,

    /// encrypt or decrypt
    #[clap(short, long)]
    decrypt: bool,

    /// Name of the input file
    #[clap(short, long, forbid_empty_values = true)]
    ifile: String,

    /// cipher phrase-file or cipher phrase
    #[clap(short, long, forbid_empty_values = true)]
    phrase: String,

    /// Name of the output file
    #[clap(short, long, required = false)]
    ofile: Option<String>,
}

/// encrypt or decrypt files
///
/// use 'mod_vig - -help' to get help
fn main() {
    let args = Args::parse();
    let ofile: String = match args.ofile {
        Some(fname) => fname,
        None => {
            let mut tmp: String = args.ifile.clone();
            if args.decrypt {
                tmp.push_str(".dec");
            } else {
                tmp.push_str(".enc");
            }
            tmp
        }
    };
    let now = Instant::now();
    if args.decrypt {
        if args.verbose {
            println!("  decrypting {} into {} ...", &args.ifile, &ofile);
        }
        let _x = modernized_vigenere::decrypt(&args.ifile, &args.phrase, &ofile);
    } else {
        if args.verbose {
            println!("  encrypting {} into {} ...", &args.ifile, &ofile);
        }
        let _x = modernized_vigenere::encrypt(&args.ifile, &args.phrase, &ofile);
    }
    if args.verbose {
        let elapsed = now.elapsed();
        println!("  Time: {} seconds", elapsed.as_secs_f64());
    }
}
