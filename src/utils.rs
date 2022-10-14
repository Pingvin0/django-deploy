use std::process::Output;
use colored::Colorize;

pub fn print_install_failure(install: &Output) {
    eprintln!(
        "{}: {}\n{}: {}",
        "STDOUT".bright_black().bold(),
        
        String::from_utf8(install.stdout.clone())
        .expect("Failed decoding stdout.")
        .bright_black(),

        "STDERR".red().bold(),
        String::from_utf8(install.stderr.clone())
        .expect("Failed decoding stderr.")
        .red()
    );
}