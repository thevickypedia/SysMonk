use std::{collections::HashMap, process::Command};

/// This module contains functions that gathers CPU brand information.
pub mod cpu_brand;
/// This module contains disk related functions.
pub mod disks;

pub mod helper;

fn get_all_disks() -> Vec<HashMap<String, String>> {
    disks::get_all_disks()
}


fn get_cpu_brand() -> Option<String> {
    cpu_brand::get_name()
}
