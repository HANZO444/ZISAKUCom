mod token;
mod tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            eprintln!("./main <source-code>");
            std::process::exit(1);
        }

        tokenizer::tokenize(args.pop().unwrap())?
    };

    for (idx, t) in tokens.iter().enumerate() {
        eprintln!("tokens[{}]: {:?}", idx, t);
    }

    Ok(())
}
