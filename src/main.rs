mod bf;

use std::{env, error::Error, ffi::OsStr, fs, path::Path, process::exit};

use bf::{bf_machine::BfMachine, bf_optimizer::BfCodeOptimizer, bf_parser::BfParser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let bf_code = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred during parsing arguments: {err}");
        exit(1);
    });
    let optimized_code = BfCodeOptimizer::optimize(&bf_code);

    let commands = BfParser::parse_compress(&optimized_code).unwrap_or_else(|err| {
        eprintln!("Error occurred during parsing Brainfuck code: {err}");
        exit(1);
    });

    let mut machine = BfMachine::default();
    machine.run(&commands).unwrap_or_else(|err| {
        eprintln!("Error occurred during runtime: {err}",);
        exit(1);
    });
}

fn parse_args(args: &[String]) -> Result<String, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Usage: bf-rust.exe [filename.(b/bf)] <--force-run>".into());
    }

    let file_path_str = &args[1];
    let force_run = args.get(2).is_some();

    let file_path = Path::new(file_path_str);
    let bf_code = fs::read_to_string(file_path)?;
    if !force_run {
        let ext = file_path
            .extension()
            .unwrap_or(OsStr::new("[no extension]"))
            .to_str()
            .unwrap();
        if ext != "b" && ext != "bf" {
            return Err(format!(
                "Unknown file extension: {}. Please provide a file with '.b' or '.bf' extension.",
                ext
            )
            .into());
        }
    }

    Ok(bf_code)
}
