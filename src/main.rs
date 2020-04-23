use device_query::{DeviceQuery, DeviceState, Keycode};

const NUM_KEYS: usize = 82;

fn main() {
    let conv = Converter::new();

    // TODO: Start up juptyer-style server with fancy visualizations?
    println!("Beginning to run... \nBest of luck!");

    // Keypress records
    let mut keypress_count: [u32 ; NUM_KEYS] = [0 ; NUM_KEYS];

    // Markov chain transition matrix
    let mut trans_mat = [[0_u32; NUM_KEYS] ; NUM_KEYS];

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

            // Add key to markov model
            for prev in &prev_keys {
                match conv.index_from_key(key) {
                    Some(curr_index) => {
                        match conv.index_from_key(prev) {
                            Some(prev_index) => {
                                trans_mat[prev_index][curr_index] += 1;
                            },
                            None => eprintln!("{:?} not found!", prev)
                        };
                    },
                    None => eprintln!("{:?} not found!", key)
                };

            }

            // Acknowledge Keypress
            if !prev_keys.contains(key) {

                // Increase the key counter
                match conv.index_from_key(key) {
                    Some(index) => keypress_count[index] += 1,
                    None => eprintln!("Key {:?} not found", key)
                };
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
    for (i, count) in keypress_count.iter().enumerate() {
        if *count > 0 {
            match conv.key_from_index(i) {
                Some(key) => println!("{:?}: {}", key, count),
                None => eprintln!("Index not found {}", i),
            };
        }
    }

    // Clean up
    println!("So long and thanks for all the fish!");
}

/// Used solely for converting between indexes and keys
struct Converter {
    pairs: [(Keycode, usize) ; NUM_KEYS]
}

impl Converter {
    pub fn new() -> Converter {
        let pairs: [(Keycode, usize); 82] = [
            (device_query::keymap::Keycode::Key0, 0),
            (device_query::keymap::Keycode::Key1, 1),
            (device_query::keymap::Keycode::Key2, 2),
            (device_query::keymap::Keycode::Key3, 3),
            (device_query::keymap::Keycode::Key4, 4),
            (device_query::keymap::Keycode::Key5, 5),
            (device_query::keymap::Keycode::Key6, 6),
            (device_query::keymap::Keycode::Key7, 7),
            (device_query::keymap::Keycode::Key8, 8),
            (device_query::keymap::Keycode::Key9, 9),
            (device_query::keymap::Keycode::A, 10),
            (device_query::keymap::Keycode::B, 11),
            (device_query::keymap::Keycode::C, 12),
            (device_query::keymap::Keycode::D, 13),
            (device_query::keymap::Keycode::E, 14),
            (device_query::keymap::Keycode::F, 15),
            (device_query::keymap::Keycode::G, 16),
            (device_query::keymap::Keycode::H, 17),
            (device_query::keymap::Keycode::I, 18),
            (device_query::keymap::Keycode::J, 19),
            (device_query::keymap::Keycode::K, 20),
            (device_query::keymap::Keycode::L, 21),
            (device_query::keymap::Keycode::M, 22),
            (device_query::keymap::Keycode::N, 23),
            (device_query::keymap::Keycode::O, 24),
            (device_query::keymap::Keycode::P, 25),
            (device_query::keymap::Keycode::Q, 26),
            (device_query::keymap::Keycode::R, 27),
            (device_query::keymap::Keycode::S, 28),
            (device_query::keymap::Keycode::T, 29),
            (device_query::keymap::Keycode::U, 30),
            (device_query::keymap::Keycode::V, 31),
            (device_query::keymap::Keycode::W, 32),
            (device_query::keymap::Keycode::X, 33),
            (device_query::keymap::Keycode::Y, 34),
            (device_query::keymap::Keycode::Z, 35),
            (device_query::keymap::Keycode::F1, 36),
            (device_query::keymap::Keycode::F2, 37),
            (device_query::keymap::Keycode::F3, 38),
            (device_query::keymap::Keycode::F4, 39),
            (device_query::keymap::Keycode::F5, 40),
            (device_query::keymap::Keycode::F6, 41),
            (device_query::keymap::Keycode::F7, 42),
            (device_query::keymap::Keycode::F8, 43),
            (device_query::keymap::Keycode::F9, 44),
            (device_query::keymap::Keycode::F10, 45),
            (device_query::keymap::Keycode::F11, 46),
            (device_query::keymap::Keycode::F12, 47),
            (device_query::keymap::Keycode::Escape, 48),
            (device_query::keymap::Keycode::Space, 49),
            (device_query::keymap::Keycode::LControl, 50),
            (device_query::keymap::Keycode::RControl, 51),
            (device_query::keymap::Keycode::LShift, 52),
            (device_query::keymap::Keycode::RShift, 53),
            (device_query::keymap::Keycode::LAlt, 54),
            (device_query::keymap::Keycode::RAlt, 55),
            (device_query::keymap::Keycode::Meta, 56),
            (device_query::keymap::Keycode::Enter, 57),
            (device_query::keymap::Keycode::Up, 58),
            (device_query::keymap::Keycode::Down, 59),
            (device_query::keymap::Keycode::Left, 60),
            (device_query::keymap::Keycode::Right, 61),
            (device_query::keymap::Keycode::Backspace, 62),
            (device_query::keymap::Keycode::CapsLock, 63),
            (device_query::keymap::Keycode::Tab, 64),
            (device_query::keymap::Keycode::Home, 65),
            (device_query::keymap::Keycode::End, 66),
            (device_query::keymap::Keycode::PageUp, 67),
            (device_query::keymap::Keycode::PageDown, 68),
            (device_query::keymap::Keycode::Insert, 69),
            (device_query::keymap::Keycode::Delete, 70),
            (device_query::keymap::Keycode::Grave, 71),
            (device_query::keymap::Keycode::Minus, 72),
            (device_query::keymap::Keycode::Equal, 73),
            (device_query::keymap::Keycode::LeftBracket, 74),
            (device_query::keymap::Keycode::RightBracket, 75),
            (device_query::keymap::Keycode::BackSlash, 76),
            (device_query::keymap::Keycode::Semicolon, 77),
            (device_query::keymap::Keycode::Apostrophe, 78),
            (device_query::keymap::Keycode::Comma, 79),
            (device_query::keymap::Keycode::Dot, 80),
            (device_query::keymap::Keycode::Slash, 81)
        ];
        Converter { pairs }
    }

    /// Get the key from the index
    pub fn index_from_key(&self, key: &Keycode) -> Option<usize> {
        for (other, index) in self.pairs.into_iter() {
            if other == key {
                return Some(*index)
            }
        }
        None
    }

    /// Get the index from the key
    pub fn key_from_index(&self, index: usize) -> Option<Keycode> {
        if index >= NUM_KEYS { 
            None
        } else {
            Some(self.pairs[index].0.clone())
        }
    }
}