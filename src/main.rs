mod funcs;

use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;




fn main() {
    loop {
        let dir_path = match funcs::get_input("Enter path you want to sort: "){
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Input error: {}",err);
                continue;
            }
        };

        let now = Instant::now();
        funcs::organize_dir(PathBuf::from(dir_path));
        println!("Time elapsed: {}s",now.elapsed().as_secs_f64());
    }
}
