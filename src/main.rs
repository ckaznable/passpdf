use std::env;
use std::io::{self, Write};

use lopdf::Document;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pdf-file>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];

    eprint!("Password: ");
    io::stderr().flush().unwrap();
    let password = rpassword::read_password().expect("Failed to read password");

    let mut doc = Document::load_with_password(input, &password)
        .expect("Failed to open PDF (wrong password or unsupported encryption)");

    let output = input
        .strip_suffix(".pdf")
        .map(|s| format!("{s}_unlocked.pdf"))
        .unwrap_or_else(|| format!("{input}_unlocked"));

    doc.save(&output).expect("Failed to save PDF");
    println!("{output}");
}
