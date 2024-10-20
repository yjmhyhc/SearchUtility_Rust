use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;
use colored::*;

fn main() {
    //capture the command line arguments
    let args: Vec<String> = env::args().collect();

    //parsing the parameters in the reverse order
    let size = args.len();
    let last_cmd_par = &args[&size-1][..];
    match last_cmd_par {
        //if it is "-h", we should print the help information
        "-h"=>{
            println!("Usage: grep [OPTIONS] <pattern> <files...>
Options:
-i                Case-insensitive search
-n                Print line numbers
-v                Invert match (exclude lines that match the pattern)
-r                Recursive directory search
-f                Print filenames
-c                Enable colored output
-h, --help        Show help information")
        }
        //if it is "-i", we should implement the case-insensitive feature
        "-i"=>{
            let _ = case_insensitive_search(&args);
        }
        //if it is "-n", we should implement the line number feature
        "-n"=>{
            let _ = search_one_file(&args[1][..], &args[2][..], true);
        }
        //if it is "-v", we should implement the inverting feature
        "-v"=>{
            let _ = inverting_search(&args);
        }
        //if it is "-r", we should implement the recursive feature
        "-r"=>{
            let _ = recursive_search(&args, false, false);
        }
        //if the last parameter is "-f", we should continue to check the second to last parameter
        "-f"=>{
            let sec_to_last_cmd_par = &args[&size-2][..];
            if sec_to_last_cmd_par=="-c" {
                //if this parameter is "-c",then we should implement three functions: recursive, coloured and file name
                let _ = recursive_search(&args, true, true);
            } else {
                //if it is not, then we only need recursive and file name
                let _ = recursive_search(&args, false, true);
            }
        }
        //if no command line parameter is included, do basic search
        &_ => {
            basic_search(&args, size);
        }
    }
}

//this function handles basic search, multiple-files is supported
fn basic_search(args: &Vec<String>, size: usize) {
    let pattern = &args[1][..];
    let files = &args[2..size];
    for file in files{
        //if we use a wildcard syntax *, all the files under this directory will be searched
        if let Some(pos) = file.find("/*") {
            let before = &file[..pos];
            //walk through the directory, set the maximum depth to 1
            for entry in WalkDir::new(before).max_depth(1).into_iter().filter_map(Result::ok) {
                //check if this entry is a file
                if entry.file_type().is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        let _ = search_one_file(&pattern, &path_str, false);
                    } else {
                        println!("Path could not be converted to a string.");
                    }
                }
            }
        } else {
            let _ = search_one_file(&pattern, &file, false);
        }
    }
}

// this function handles case insensitive search
fn case_insensitive_search(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let pattern = &args[1][..];
    let file = &args[2][..];
    //open the file
    let path = Path::new(file);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    //iterate through every line
    for line in reader.lines() {
        let line = line?; //handle possible errors
        //check if this line contains the pattern string
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            println!("{}", line);
        }
    }
    Ok(())
}

//this function handles inverting search(print the lines that do not contain the pattern)
fn inverting_search(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let pattern = &args[1][..];
    let file = &args[2][..];
    //open the file
    let path = Path::new(file);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    //iterate through every line
    for line in reader.lines() {
        let line = line?; //handle possible errors
        //check if this line does not contain the pattern string
        if !line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

/*
    this function searches one directory recursively,
    could change the color of the matched text to red if c is true
    could print the file name if f is true,
 */
fn recursive_search(args: &Vec<String>, c: bool, f: bool) -> Result<(), Box<dyn std::error::Error>>{
    let pattern = &args[1][..];
    let directory = &args[2][..];
    match (c,f) {
        (true, true) => {
            for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
                //check if this entry is a file
                if entry.file_type().is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        //open the file
                        let path = Path::new(path_str);
                        let file = File::open(&path)?;
                        let reader = io::BufReader::new(file);

                        //iterate through every line
                        for line in reader.lines() {
                            let line = line?; //handle possible errors
                            //check if this line contains the pattern string
                            if line.contains(pattern) {
                                print!("{}: ", path_str.replace("\\", "/"));
                                println!("{}", line.replace(pattern, &pattern.red().to_string()));
                            }
                        }
                    } else {
                        println!("Path could not be converted to a string.");
                    }
                }
            }
        }
        (true, false) => {
            for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
                //check if this entry is a file
                if entry.file_type().is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        //open the file
                        let path = Path::new(path_str);
                        let file = File::open(&path)?;
                        let reader = io::BufReader::new(file);

                        //iterate through every line
                        for line in reader.lines() {
                            let line = line?; //handle possible errors
                            //check if this line contains the pattern string
                            if line.contains(pattern) {
                                println!("{}", line.replace(pattern, &pattern.red().to_string()));
                            }
                        }
                    } else {
                        println!("Path could not be converted to a string.");
                    }
                }
            }
        }
        (false, true) => {
            for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
                //check if this entry is a file
                if entry.file_type().is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        //open the file
                        let path = Path::new(path_str);
                        let file = File::open(&path)?;
                        let reader = io::BufReader::new(file);

                        //iterate through every line
                        for line in reader.lines() {
                            let line = line?; //handle possible errors
                            //check if this line contains the pattern string
                            if line.contains(pattern) {
                                print!("{}: ", path_str.replace("\\", "/"));
                                println!("{}", line);
                            }
                        }
                    } else {
                        println!("Path could not be converted to a string.");
                    }
                }
            }
        }
        (false, false) => {
            for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
                //check if this entry is a file
                if entry.file_type().is_file() {
                    if let Some(path_str) = entry.path().to_str() {
                        //open the file
                        let path = Path::new(path_str);
                        let file = File::open(&path)?;
                        let reader = io::BufReader::new(file);

                        //iterate through every line
                        for line in reader.lines() {
                            let line = line?; //handle possible errors
                            //check if this line contains the pattern string
                            if line.contains(pattern) {
                                println!("{}", line);
                            }
                        }
                    } else {
                        println!("Path could not be converted to a string.");
                    }
                }
            }
        }
    }
    Ok(())
}

/*
    this function searches one file,
    could print the line number if n is true,
    could change the color of the matched text to red if c is true
 */
fn search_one_file(pattern: &str, file: &str, n: bool) -> Result<(), Box<dyn std::error::Error>>{
    if n {
        //open the file
        let path = Path::new(file);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        //iterate through every line
        let mut i = 1;
        for line in reader.lines() {
            let line = line?; //handle possible errors
            //check if this line contains the pattern string
            if line.contains(pattern) {
                println!("{}: {}", i, line);
            }
            i+=1;
        }
    }else {
        //open the file
        let path = Path::new(file);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        //iterate through every line
        for line in reader.lines() {
            let line = line?; //handle possible errors
            //check if this line contains the pattern string
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
    }
    Ok(())
}
