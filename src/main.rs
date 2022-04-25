//! simple cli-Interface for the modernized _vigenere lib
//!
//! encrypt and decrypt any files with a vigenere cipher based on bytes ( not on teh alphabet )
use modernized_vigenere;

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
    #[clap(short, long)]
    ifile: String,

    /// cipher phrase
    #[clap(short, long)]
    phrase: String,

    /// Name of the output file
    #[clap(short, long)]
    ofile: String,
}

/// encrypt or decrypt files
///
/// use 'mod_vig - -help' to get help
fn main() {
    let args = Args::parse();
    if args.phrase.is_empty() {
        println!("  Must provide a non-empty cipher-phrase. Exiting ...");
        return;
    }
    let now = Instant::now();
    if args.decrypt {
        if args.verbose {
            println!("  decrypting {} into {} ...", args.ifile, args.ofile);
        }
        let _x = modernized_vigenere::decrypt(&args.ifile, &args.phrase, &args.ofile);
    } else {
        if args.verbose {
            println!("  encrypting {} into {} ...", args.ifile, args.ofile);
        }
        let _x = modernized_vigenere::encrypt(&args.ifile, &args.phrase, &args.ofile);
    }
    if args.verbose {
        let elapsed = now.elapsed();
        println!("  Time: {} seconds", elapsed.as_secs_f64());
    }
}
