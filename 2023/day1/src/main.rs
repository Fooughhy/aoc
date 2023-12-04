// Globals settings
#![allow(dead_code)]
#![allow(unused)]

// Modules
pub mod file_manip;
pub mod calibration_object;

// mod file::file;

use std::{env::{self, ArgsOs}, fs::File, io::{BufReader, Read, IoSliceMut, BufRead, Lines}, str::Chars};
use std::iter::Filter;

use crate::calibration_object::LineFilter;
// use crate::calibration_object::CalibrationDocument;

fn main()
{
    let args: ArgsOs = env::args_os();

    let mut doc_vec: Vec<calibration_object::CalibrationDocument> = Vec::new();
    // for arg in args
    for arg in args
    {
        println!("{arg:?}");
        file_manip::print_lines_in_file(&arg);
        let file = File::open(&arg).unwrap();
        let mut buf_reader = BufReader::new(file);


        // let mut contents = String::new();
        // let mut lines: IoSliceMut<String> = IoSliceMut::new();
        // buf_reader.read_to_string(&mut contents).unwrap();
        // buf_reader.read_vectored(&mut lines).unwrap();
        // doc_vec.push(file, file.)
        
        let mut line_vec: Vec<String> = Vec::new();
        
        // let mut contents = String::new();
        // let mut current_line: &mut String = lines.first_mut();
        // loop {
        //     let mut contents = String::new();
        //     // buf_reader.read_line(current_line.as_mut());
        //     let result = buf_reader.read_line(&mut contents);
            
        //     // current_line = contents;

        //     if result.is_err()
        //     {
        //         break;
        //     }
        //     else 
        //     {
        //         line_vec.push(contents);
        //     }
        // }

        for line in buf_reader.lines()
        {
            if line.is_ok()
            {
                line_vec.push(line.unwrap());
            }
            else
            {
                break;
            }
        }
        // let lines: Lines<String>;

        // buf_reader.
        
        let mut cal_doc: calibration_object::CalibrationDocument = calibration_object::CalibrationDocument { lines: line_vec , filtered_lines: Vec::new()};

        for line in cal_doc.lines
        {
            // Print each line
            println!("{line:?}");
            cal_doc.filtered_lines.push(line.matches(char::is_numeric).collect())
        }

        for line in &cal_doc.filtered_lines
        {
            // Print numbers on each line
            println!("{line:?}");
        }

        for line in &cal_doc.filtered_lines
        {
            // Print numbers on each line
            if line.len() > 1
            {
                let mut characters: Chars = line.chars();
                println!("{}{}", characters.next().unwrap(), characters.next_back().unwrap());
            }
        }
    }

}
