//rlarm
//TO-DO
//add GUI

use std::fs::File;
use std::io::{BufReader, stdin};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};

//function to get the amount of time needed for waiting
fn get_work_time() -> u64 {

    println!("Work Time: "); 
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    
    let work_time: u64 = input.trim().parse().unwrap();

    return work_time;
}

fn user_interupt(interrupted: Arc<Mutex<bool>>) {

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
}


fn play_track(interrupted: Arc<Mutex<bool>>) {

        // Clone a reference to the flag for the thread
    let interrupted_clone = Arc::clone(&interrupted);

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("sound/soft.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    sink.append(source);
    while !*interrupted_clone.lock().unwrap() {
        thread::sleep(Duration::from_millis(100));
    }
    sink.stop();
} 

fn main() {

    let work_time: u64 = get_work_time();

    println!("See you then.");

    //wait for that amount of time 
    thread::sleep(Duration::from_secs(work_time));
    
    // Create a shared flag to signal interruption
    let interrupted = Arc::new(Mutex::new(false));

    //start thread waiting for user input when input is entered interrupted = true
    user_interupt(Arc::clone(&interrupted));
    //start audio thread and stop when interrupted = true
    play_track(Arc::clone(&interrupted));

    println!("Press Enter to stop.");
    // Sleep until interrupted or a timeout of 5 seconds
    while !*interrupted.lock().unwrap() {
        thread::park_timeout(Duration::from_secs(1)); // Check every second
    }

    println!("rest time");
}
