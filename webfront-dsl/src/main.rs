mod tokenizer;
mod token;
mod lexicon;
mod commands;
use commands::CommandList;

fn main() {
    let command_list = CommandList::new();

    //Find all our args, skip the path we're executing from.
    let mut args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    args = args.into_iter().skip(1).take(args_len+1).collect();

    if args.len() == 0 {
        println!("wf.exe greets you, perhaps try a command, like:\n\twf.exe --tests\n\twf.exe --compile <file>\nBye, for now (^.^)/");
        return;
    } 

    let identifier = args.get(0).unwrap();
    command_list.run(identifier, args.clone());
}