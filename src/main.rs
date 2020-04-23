use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {

    // TODO: Start up juptyer-style server with fancy visualizations?
    println!("Beginning to run... \nBest of luck!");

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

        // O(n*m), 
        //      n = # of keys pressed now
        //      m = # of keys pressed previously
        // I don't intend to press >3 keys at any given time,
        // so I'm not that concerned about this
        for key in &curr_keys {

            // Acknowledge Keypress
            if !prev_keys.contains(key) {
                println!("{:?} key pressed!", *key);
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

    // Clean up
    println!("So long and thanks for all the fish!");
}
