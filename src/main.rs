use std::env;
use std::fs;
use std::io::{self, Write};
use rand::Rng; // Import the rand crate

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the help option is provided
    if args.contains(&String::from("-h")) {
        print_help();
        std::process::exit(0);
    }

    // Check if the correct number of arguments is provided
    if args.len() != 2 && !args.contains(&String::from("-v")) {
        eprintln!("Usage: {} [-h] [-v] [file_path]", args[0]);
        std::process::exit(1);
    }

    // Extract the file path from the command-line arguments
    let file_path = if args.len() == 2 {
        &args[1]
    } else {
        args.last().unwrap()
    };

    // Check for the verbose option
    let verbose_mode = args.contains(&String::from("-v"));

    match delete_file(file_path, verbose_mode) {
        Ok(_) => println!("File securely deleted."),
        Err(err) => eprintln!("Error securely deleting file: {}", err),
    }
}

fn delete_file(file_path: &str, verbose_mode: bool) -> Result<(), io::Error> {
    // Generate a buffer of random data to overwrite the file
    let mut rng = rand::thread_rng();
    let buffer_size = 1024; // Choose an appropriate buffer size
    let random_buffer: Vec<u8> = (0..buffer_size).map(|_| rng.gen()).collect();

    // Securely open the file and overwrite its content with random data
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(file_path)?;

    // Get the file size and calculate the number of times to overwrite
    let file_size = file.metadata()?.len();
    let overwrite_count = (file_size as f64 / buffer_size as f64).ceil() as usize;

    // Display additional verbose information before overwriting
    if verbose_mode {
        println!("Verbose Mode:");
        println!(" - File: {}", file_path);
        println!(" - Initial File Size: {} bytes", file_size);
        println!(" - Buffer Size: {} bytes", buffer_size);
        println!(" - Overwrite Count: {}", overwrite_count);
    }

    for _ in 0..overwrite_count {
        file.write_all(&random_buffer)?;
    }

    // Write any remaining bytes if the file size is not a multiple of buffer_size
    let remaining_bytes = (file_size % buffer_size as u64) as usize;
    if remaining_bytes > 0 {
        let additional_random_buffer: Vec<u8> =
            (0..remaining_bytes).map(|_| rng.gen()).collect();
        file.write_all(&additional_random_buffer)?;
    }

    // Display additional verbose information after overwriting
    if verbose_mode {
        println!(" - Remaining Bytes After Overwrite: {}", remaining_bytes);
    }

    // Delete the file from the file system
    fs::remove_file(file_path)?;

    Ok(())
}

fn print_help() {
    println!("Usage:");
    println!("  -h      Display this help menu");
    println!("  -v      Enable verbose mode");
    println!("  [file_path]   Path to the file to be securely deleted");
}
