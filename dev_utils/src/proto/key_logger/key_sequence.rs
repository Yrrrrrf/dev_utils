#![allow(unused)]  // Allow dead code in a file or globally


// std
use std::io::Write;
// crate
use crate::util::terminal;
// extern 
use rand::Rng;
use chrono::{DateTime, Utc, NaiveTime, Duration};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode, device_state};
use std::thread;
use std::time;  // Time also has a Duration struct, but it's different from chrono::Duration (only in methods)


/// The UserInputKeySequence struc is used to store a sequence of keys
/// 
/// It stores the time when the key is pressed and unpressed
/// 
/// ### Attributes
/// - init_time [`DateTime<Utc>`] - The time when the sequence is created
/// - sequence [`Vec<(Keycode, bool, NaiveTime)>`] - The sequence of keys
///    - [`Keycode`] is the key pressed
///    - [`NaiveTime`] is the time when the key is pressed
///    - [`bool`] is true if the key is pressed and false if the key is unpressed
#[derive(Debug, Clone, Default)]
pub struct UserInputKeySequence {
    init_time: DateTime<Utc>,
    sequence: Vec<(Keycode, bool, NaiveTime)>
}


impl UserInputKeySequence {
    /// Create a new instance of UserInputKeySequence
    fn new() -> Self {
        println!("New Default {}", terminal::set_fg("UserInputKeySequence", "g"));  // print the current time
        Self {
            // sequence: Vec::new(),  // create a new vector of Keycode (empty)
            init_time: Utc::now(),  // set the init time
            ..Default::default()  // Default value allow us to create a new UserInputKeySequence without the init_time
        }
    }


    /// Add a key to the sequence
    /// 
    /// ### Arguments
    /// 
    /// - key [`Keycode`] - The key pressed
    /// - is_pressed [`bool`] - True if the key is pressed and false if the key is unpressed
    /// 
    /// ### Example
    /// ```
    /// let mut key_sequence: UserInputKeySequence = UserInputKeySequence::new();
    /// 
    /// key_sequence.add_key(Keycode::A, true);
    /// 
    /// assert_eq!(key_sequence.sequence[0].0, Keycode::A);
    /// assert_eq!(key_sequence.sequence[0].1, true);
    /// ```
    fn add_key(&mut self, key: Keycode, is_pressed: bool) {
        self.sequence.push((key, is_pressed, Utc::now().time()));
    }


    /// Start the key sequence. It will stop when the escape key is pressed
    /// 
    /// ### Parameters
    /// - self [`&mut Self`] - The UserInputKeySequence
    fn run(&mut self) {
        let mut old_keys: Vec<Keycode> = Vec::new();  // create a new vector of Keycode (empty)

        self.init_time = Utc::now();  // set the init time
        println!("    {} {}", terminal::set_fg("Init Time", "g"), self.init_time);  // print the current time

        loop {
            // ? Keyboard events
            let new_keys: Vec<Keycode> = DeviceState::new().get_keys(); // get the current pressed keys
            if new_keys.contains(&Keycode::Escape) {break;}  // if escape is pressed, then break the loop

            // if the new_keys and the old_keys are different, then add the new keys to the sequence
            new_keys.iter().for_each(|key| if !old_keys.contains(key) {self.add_key(*key, true)});  // add a key when it's   PRESSED
            old_keys.iter().for_each(|key| if !new_keys.contains(key) {self.add_key(*key, false)});  // add a key when it's UNPRESSED
            // update the old keys
            old_keys = new_keys.clone();  // update the old keys


            // TODO: return self.sequence

        }
    }


    /// Get the time between all the keys
    /// 
    /// ### Parameters
    /// - self [`&Self`] - The UserInputKeySequence
    /// 
    /// ### Returns
    /// - time_between_keys [`Vec<Duration>`] - The time between all the keys
    fn get_time_between_keys(&self) -> Vec<Duration> {
        let mut time_between_keys: Vec<Duration> = Vec::new();  // create a new vector of Keycode (empty)
        time_between_keys.push(self.sequence[0].2.signed_duration_since(self.init_time.time()));  // add the time between init_time and the first key
        self.sequence.iter().enumerate().for_each(|(i, (_, _, time))| if i != 0 {time_between_keys.push(time.signed_duration_since(self.sequence[i-1].2))});
        time_between_keys
    }


    /// Print the sequence in the terminal
    /// 
    /// Print it in a table format
    /// 
    /// | i | key | is_pressed | act_hour | time_between_last_key |
    /// |-|-|-|-|-|
    /// | 0 | A   | ↓          | 12:00:00.000000000 | 00:00:00.000000000 |
    fn print_sequence(&self) {
        if self.sequence.len() == 0 {return;}  // if the sequence is empty, then return
        println!("    {} {}", terminal::set_fg("Init Time", "g"), self.init_time);  // print the current time
        let time_between_keys = self.get_time_between_keys();
        self.sequence.iter().enumerate().for_each(|(i, (key, is_pressed, time))|println!("i{:_>4} {:>16} {} {:<20} {}", i, key.to_string(), if *is_pressed {terminal::set_fg("↓", "r")} else {terminal::set_fg("↑", "g")}, time.to_string(), self.get_time_between_keys()[i]));
    }


    /// Save the sequence as a csv file
    /// 
    /// ### Parameters
    /// - self [`&Self`] - The UserInputKeySequence
    ///     
    fn save_as_csv(&self, file_name: &str) {
        // Store the stuct instanse with the csv format
        let mut csv = String::new();
        csv.push_str("# This file is generated by the key_sequence module\n");  // add a comment to the csv file
        csv.push_str(&format!("# Init time: {}\n", self.init_time));  // add the header that contains the init time

        csv.push_str("i,key,is_pressed,act_hour,time_between_last_key\n");
        // add the sequence to the csv file
        self.sequence.iter().enumerate().for_each(|(i, (key, is_pressed, time))| csv.push_str(&format!("{},{},{},{},{}\n", i, key.to_string(), if *is_pressed {"↓"} else {"↑"}, time.to_string(), self.get_time_between_keys()[i])));

        // ~ Provitional name
        let file_name = format!("{}.csv", file_name);  // create the file name with the current time
        // let file_name = format!("{}.csv", self.init_time.to_string().replace(":", "_").replace(".", "_"));  // create the file name with the current time
        let mut file = std::fs::File::create(&file_name).expect("Unable to create file");  // create the file
        file.write_all(csv.as_bytes()).expect("Unable to write data");  // write the data to the file

        println!("{} - {}", terminal::set_fg(&file_name, "c"), terminal::set_fg("Saved successfully", "g"));
    }


    /// Save the sequence as a json file
    /// 
    /// ### Parameters
    /// - self [`&Self`] - The UserInputKeySequence
    /// - file_name [`&str`] - The name of the file
    fn save_as_json(&self, file_name: &str) {
        // Store the stuct instanse with the json format
        let mut json = String::new();
        json.push_str("{\n");  // add the header
        json.push_str(&format!("    \"init_time\": \"{}\",\n    \"sequence\": [\n", self.init_time));  // add the init time
        // add the sequence to the json file
        self.sequence.iter().enumerate().for_each(|(i, (key, is_pressed, time))| json.push_str(&format!("        {{\n            \"key\": \"{}\",\n            \"is_pressed\": {},\n            \"time\": \"{}\"\n        }}{}", key.to_string(), if *is_pressed {"true"} else {"false"}, time.to_string(), if i != self.sequence.len()-1 {",\n"} else {"\n"})));
        json.push_str("    ]\n}\n");  // add the footer
        
        // ~ Provitional name
        let file_name = format!("{}.json", file_name);  // create the file name with the current time
        let mut file = std::fs::File::create(&file_name).expect("Unable to create file");  // create the file
        file.write_all(json.as_bytes()).expect("Unable to write data");  // write the data to the file

        println!("{} - {}", terminal::set_fg(&file_name, "c"), terminal::set_fg("Saved successfully", "g"));
    }


    /// Return the time that a certain key was pressed
    /// 
    /// If the key is not pressed, then return an empty vector
    /// If the key is pressed multiple times, then return a vector with all the times
    /// 
    /// ### Parameters
    /// - self [`&Self`] - The UserInputKeySequence
    /// - key [`Keycode`] - The key pressed
    fn get_time_pressed(&self, key: Keycode) -> Vec<Duration> {
        let mut time_pressed: Vec<Duration> = Vec::new();  // create a new vector of Keycode (empty)
        // for all the keys in the sequence
        for (i, (key_, is_pressed, time)) in self.sequence.iter().enumerate() {  // iterate over the sequence
            // if the key is pressed, check until the key is unpressed
            if *key_ == key && *is_pressed {
                let mut j = i + 1;  // set the index to the next key
                while j < self.sequence.len() && self.sequence[j].0 == key && self.sequence[j].1 {j += 1;}  // check until the key is unpressed
                time_pressed.push(self.sequence[j-1].2.signed_duration_since(*time));  // add the time pressed to the vector
                

            }

        }
        time_pressed
    }


}


/// Test the module
pub fn test() {

    let mut key_sequence: UserInputKeySequence = UserInputKeySequence::new();

    // for _ in 0..10 {
    //     key_sequence.add_key(  // Add a key to the sequence
    //         match rand::thread_rng().gen_range(0..=255) {  // Generate a random number between 0 and 255
    //             0..=25 => Keycode::A,
    //             26..=50 => Keycode::B,
    //             51..=75 => Keycode::C,
    //             76..=100 => Keycode::D,
    //             101..=125 => Keycode::E,
    //             126..=150 => Keycode::F,
    //             151..=175 => Keycode::G,
    //             176..=200 => Keycode::H,
    //             201..=225 => Keycode::I,
    //             226..=250 => Keycode::J,
    //             251..=255 => Keycode::K,
    //             _ => Keycode::A
    //         },
    //         rand::thread_rng().gen_bool(0.5));  // Generate a random boolean
    // }

    // key_sequence.add_key(Keycode::A, true);  // Press the A key
    // thread::sleep(time::Duration::from_millis(1000));  // wait 1 second
    // key_sequence.add_key(Keycode::A, false);  // Unpress the A key
    // key_sequence.add_key(Keycode::A, true);  // Press the A key
    // key_sequence.add_key(Keycode::A, false);  // Unpress the A key
    
    // let timed_key = Keycode::A;  // Press the A key
    
    // println!("{} pressed {:?} seconds", timed_key, key_sequence.get_time_pressed(timed_key));  // Print the time that the key was pressed
    
    key_sequence.run();  // Start the key sequence

    key_sequence.print_sequence();  // Print the key sequence

    key_sequence.save_as_csv(&format!("{}{}", DATA_PATH, "test"));  // Save the key sequence as a csv file
    key_sequence.save_as_json(&format!("{}{}", DATA_PATH, "test"));  // Save the key sequence as a csv file
}

