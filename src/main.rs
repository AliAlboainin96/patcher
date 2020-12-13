#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate glob;
use glob::glob;
use std::env;
use std::fs;
use std::process::Command;

#[macro_export]
macro_rules! cd {
    () => {
        println!("Empty...");
    };

    ($val:expr) => {
        std::env::set_current_dir($val).expect("failed to change directory");
    };
}
pub fn patcher(Target: &String, Directory: &String) {
    let currentPath = fs::read_dir(Directory).unwrap();

    let mut FilesVec = Vec::new();

    // Iterate over files and push them to the vector

    for file in currentPath {
        //FilesVec.push(file.unwrap().path().display());
        FilesVec.push(file.unwrap().path().display());
        println!("Patch: {}", FilesVec[]);
    }

    // for patches in 0..FilesVec.len() {
    //Command::new("patch")
    //   .args(&["-u", Target, "-i", FilesVec[index].to_string()])
    // .spawn()
    //.expect("failed to patch process");

    //     println!("{}", FilesVec[1]);
    //}
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let pTarget = &args[1];
    let pDir = &args[2];

    patcher(pTarget, pDir);
    //patch();
}
