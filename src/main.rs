use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {

    //get input for time to wait 
    println!("Enter the duration you want to work in minutes:");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let work_duration: u64 = input.trim().parse().unwrap(); 

    //wait for that amount of time 
    thread::sleep(Duration::from_secs(work_duration));
    

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("sound/soft.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // Create a shared flag to signal interruption
    let interrupted = Arc::new(Mutex::new(false));

    // Clone a reference to the flag for the thread
    let interrupted_clone = Arc::clone(&interrupted);

    // Spawn a thread to handle user input
    thread::spawn(move || {
        // Wait for user input to interrupt the sleep
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Set the flag to true to signal interruption
        *interrupted_clone.lock().unwrap() = true;
    });

    // Sleep until interrupted or a timeout of 5 seconds
    while !*interrupted.lock().unwrap() {
        thread::park_timeout(Duration::from_secs(1)); // Check every second
    }

    println!("rest time");
}
