//! This is a library that contains functions related to
//! dealing with processes,
//! and makes these tasks more convenient.

use std::process;

/// This function gets the process ID of the current
/// executable. It returns a non-zero number
pub fn get_process_id() -> u32 {
    process::id()
}