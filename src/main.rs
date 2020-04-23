use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;


fn main() {

    // TODO: Start up juptyer-style server with fancy visualizations?
    println!("Beginning to run... \nBest of luck!");

    // Keypress records
    let mut keypress_count: HashMap<String, usize> = HashMap::new();

    // Init previous keys vector
    let mut prev_keys: Vec<Keycode> = Vec::new();

    // Temporary hotkey combination to exit keylogging gracefully
    let mut lctrl = false;
    let mut lshift = false;
    let mut q = false;

    // Until exit condition
    loop {

        // Current keys
        let curr_keys = DeviceState::new().get_keys();

        // Foreach key currently pressed
        for key in &curr_keys {

            // Acknowledge Keypress
            if !prev_keys.contains(key) {
                println!("{:?} key pressed!", *key);

                // Increase the key counter
                let lookup = format!("{:?}", *key);
                let counter = keypress_count.entry(lookup).or_insert(0);
                *counter += 1;
            }

            // Exit hotkey sequence
            match key {
                &Keycode::LControl => lctrl = true,
                &Keycode::LShift => lshift = true,
                &Keycode::Q => q = true,
                _ => {}
            }
        }

        // Update previous keys
        prev_keys = curr_keys;

        // Check for exit condition
        if lctrl && lshift && q {
            break;
        } else {
            lctrl = false;
            lshift = false;
            q = false;
        }
    }

    // Display keypresses
    for (key, count) in &keypress_count {
        println!("{}: {}", key, count);
    }

    // Clean up
    println!("So long and thanks for all the fish!");
}