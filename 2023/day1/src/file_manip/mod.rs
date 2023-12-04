// pub mod file_manip
// {
use std::fs::{self, File};
use std::path::Path;

use std::ffi::{OsStr, OsString};
fn verify_file_path(path:&OsString) -> bool
{
    // let path_to_verify = path.as_os_str();

    // let mut file = File::open(path)?;
    let file_exists = Path::new(&path).exists();

    // let error:Result<Vec<u8>, std::io::Error> = fs::read(path);
    // error

    file_exists
}

pub fn print_lines_in_file(path:&OsString)
{
    // if verify_file_path(&path)
    // {
    println!("{:#?}", verify_file_path(&path));
    println!("{:#?}", path.to_str().unwrap());
}

// }