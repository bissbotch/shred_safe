use std::env;
use std::fs;
use std::io::{self, Write};
use rand::Rng; // Import the rand crate

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Extract the file path from the command-line arguments
    let file_path = &args[1];

    match delete_file(file_path) {
        Ok(_) => println!("File securely deleted."),
        Err(err) => eprintln!("Error securely deleting file: {}", err),
    }
}

fn delete_file(file_path: &str) -> Result<(), io::Error> {
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

    for _ in 0..overwrite_count {
        file.write_all(&random_buffer)?;
    }

    // Write any remaining bytes if the file size is not a multiple of buffer_size
    let remaining_bytes = (file_size % buffer_size as u64) as usize;
    if remaining_bytes > 0 {
        let additional_random_buffer: Vec<u8> = (0..remaining_bytes).map(|_| rng.gen()).collect();
        file.write_all(&additional_random_buffer)?;
    }

    // Delete the file from the file system
    fs::remove_file(file_path)?;

    Ok(())
}
