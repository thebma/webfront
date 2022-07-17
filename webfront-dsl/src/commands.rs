use std::fs;
use std::path::Path;

use chrono::Utc;

use crate::tokenizer::Tokenizer;
use crate::token::Token;

trait Command {
    fn is_this(&self, identifier: &str) -> bool;
    fn execute(&mut self, args: &Vec<String>);
}

#[derive(Clone)]
struct CompileCommand {
    pub result_tokens: Vec<Token>
}

impl Command for CompileCommand {
    fn is_this(&self, identifier: &str) -> bool {
        return identifier == "--compile";
    }

    fn execute(&mut self, args: &Vec<String>) {
        if args.len() < 2 {
            panic!("No args for the file to compile was given.");
        }

        let file_path = args.get(1).unwrap();
        println!("Compiling file {}...", file_path);

        let source = fs::read_to_string(file_path)
            .expect(["Could not read source for", file_path].join(" ").as_str());
        let mut tokenizer = Tokenizer::new(&source);
        self.result_tokens = tokenizer.tokenize();
    }
}

struct RunTestsCommand;
impl Command for RunTestsCommand {
    fn is_this(&self, identifier: &str) -> bool {
        return identifier.trim() == "--tests";
    }

    fn execute(&mut self, _: &Vec<String>) {
        println!("Running tests, detecting test file...");

        //Find our path.
        let test_folder_path = Path::new("./tests");
        if !test_folder_path.is_dir() {
            println!("No tests to run, tests folder was not found.");
            return;
        }

        //Scan the directory for test files.
        let mut found_tests = Vec::new();
        let paths = fs::read_dir(test_folder_path).unwrap();
    
        for path in paths {
            found_tests.push(path.unwrap().path());
        }
        
        //Create a new Run command for all our test cases.
        for file in found_tests {
            let file_path_clone = file.clone();
            println!("Found test file at {:?}", file);

            let mut command = CompileCommand { result_tokens: Vec::new() };
            let command_args: Vec<String> = vec![String::new(), file.into_os_string().into_string().unwrap()];
            command.execute(&command_args);

            let command_clone = command.clone();
            let result = command_clone.result_tokens;

            println!("Compiled {:?} with {:?} tokens", file_path_clone, result)
        }
    }
}

pub struct DumpTokensCommand;

impl Command for DumpTokensCommand {
    fn is_this(&self, identifier: &str) -> bool {
        return "--dump".eq(identifier);
    }

    fn execute(&mut self, args: &Vec<String>) {
        if args.len() < 2 {
            panic!("No args for the file to compile was given.");
        }

        let file_path = args.get(1).unwrap();
        println!("Dumping file {}...", file_path);

        let mut compile_command = CompileCommand { result_tokens: Vec::new() };
        compile_command.execute(&args);

        let tokens = compile_command.result_tokens;
        
        let dump_string: String = tokens
            .into_iter()
            .map(|token| { return [token.word.to_string(), format!("{:?}", token.state)].join(" :: "); } )
            .collect::<Vec<String>>()
            .join("\n");

        fs::create_dir_all("/dumps/")
            .expect("Could not create dumps folder.");

        fs::write(format!("/dumps/tokens{:?}.", Utc::now().timestamp()), dump_string)
            .expect("Could not create dump file.");
    }
}

pub struct CommandList {
    commands: Vec<Box<dyn Command>>
}

impl CommandList {
    pub fn new() -> Self {
        let mut commands_vector = Vec::<Box<dyn Command>>::new();
        commands_vector.push(Box::new(CompileCommand { result_tokens: Vec::new() }));
        commands_vector.push(Box::new(RunTestsCommand { }));
        commands_vector.push(Box::new(DumpTokensCommand { }));

        return CommandList {
            commands: commands_vector
        }
    }

    pub fn run(self, identifier: &str, args: Vec<String>) {
        self.commands.into_iter()
            .filter(|cmd| cmd.is_this(identifier))
            .for_each(|mut cmd| cmd.execute(&args));
    }
}