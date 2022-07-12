mod tokenizer;
mod token;
mod lexicon;

use std::fs;
use std::path::PathBuf;
use std::path::Path;

use tokenizer::Tokenizer;

fn main() {
    //TODO AB 26/6: Empty args will still trigger the _ branch of this match.
    //TODO AB 26/6: Deal with file name, so we can run "--run <file>.wf", there are 2 different arguments.
    for arg in std::env::args() {
        if arg.contains("wf.exe") {
            continue;
        }

        match arg.as_str() {
            "--run" => println!("not implemented yet, whoops."),
            "--tests" => run_tests(),
            _ => println!("Argument {} was unknown", arg)
        }
    }
}

fn execute_file(file: PathBuf) {
    let source = fs::read_to_string(file).unwrap();
    let mut tokenizer = Tokenizer::new(&source);
    tokenizer.tokenize();
}

fn run_tests() {
    println!("Running tests, detecting test file...");
    let test_folder_path = Path::new("./tests");

    if !test_folder_path.is_dir() {
        println!("No tests to run, tests folder was not found.");
        return;
    }

    let test_files = scan_test_directory(&test_folder_path);

    for file in test_files {
        println!("Found test file at {:?}", file);
        execute_file(file);
    }

}

fn scan_test_directory(test_directory: &Path) -> Vec<PathBuf> {
    let mut found_files = Vec::new();

    let paths = fs::read_dir(test_directory).unwrap();

    //TODO AB 26/6: Deal with recursiveness, sub directories are ignore atm.
    for path in paths {
        found_files.push(path.unwrap().path());
    }


    return found_files;
}
