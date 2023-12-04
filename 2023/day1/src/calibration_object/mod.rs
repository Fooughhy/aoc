use std::io::Lines;

pub trait LineFilter<T>
{
    // fn replace_func(inputChar: char, outputChar: char);
    fn filter_func(line: T) -> bool;
}

pub struct CalibrationDocument
{
    // pub file: std::fs::File,
    pub lines: Vec<String>,
    pub filtered_lines: Vec<String>,
}
// pub struct CalibrationDocument<T>
// {
//     pub file: std::fs::File,
//     pub lines: Lines<T>,
// }

impl LineFilter<String> for CalibrationDocument
{
    // fn filter_func(line: String) -> bool
    // {
    //     return line.replace("h");
    // }

    fn filter_func(line: String) -> bool
    {
        return true;
        // return line.chars().into_iter().map(|character| if !character.is_numeric(){character} else {''})
    }
}