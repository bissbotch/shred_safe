use std::env;
use std::fs;
use std::io::{self, Write};
use rand::Rng; // Import the rand crate

fn main() {
    // ... (unchanged main function)

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

    // Get the file size and determine the number of times to overwrite
    let file_size = file.metadata()?.len();
    let overwrite_count = (file_size as f64 / buffer_size as f64).ceil() as usize;

    for _ in 0..overwrite_count {
        file.write_all(&random_buffer)?;
    }

    // Delete the file from the file system
    fs::remove_file(file_path)?;

    Ok(())
}
