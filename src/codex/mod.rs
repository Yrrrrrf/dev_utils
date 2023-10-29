//! ENCODE & DECODE
//! 
//! This file contains the functions to encode and decode a string.
//! 
//! The encoding is done using the Base64 algorithm.
//! 
// todo: Add a feature flag to enable/disable this module
// #![cfg(feature = "codex")]  // Only compile this module if the "codex" feature is enabled


/// Encode data using the Gzip algorithm.
/// 
/// The Gzip algorithm is used to compress data.
/// It is used in the HTTP protocol to compress data before sending it to the client.
pub mod gzip;

/// Encode a string using the Base64 algorithm.
/// 
/// The Base64 algorithm is used to encode binary data as a string.
/// It is used in the HTTP protocol to encode the username and password in the Authorization header.
pub mod base64;
