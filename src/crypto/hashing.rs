//! Hashing functions.
//! 
//! This module contains a hashing function for the project.
//! 
//! The hash is a function that maps a string of any length to a fixed-length string.
//! This is a one-way function, i.e. it is not possible to find the original string from the hash.
//! 
//! So the hash is used to verify the integrity of the data.
//! It means that if the hash of the data is the same, then the data has not been modified.
//! If the hash is different, then the data has been modified.
//! 
//! The hash is also used to **verify the authenticity of the data**.
//! 

use std::collections::HashMap;
use std::hash::{Hash, Hasher};


/// Hashing function.
/// 
/// This function maps a string of any length to a fixed-length string.
/// This is a one-way function, i.e. it is not possible to find the original string from the hash.
/// 
/// So the hash is used to verify the integrity of the data.
pub fn hash<T: Hash>(t: &T) -> u64 {
    // Create a hasher
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);  // Hash the data. This will return a u64 hash value that is unique to the data.
    s.finish()  // Return the hash value
}
