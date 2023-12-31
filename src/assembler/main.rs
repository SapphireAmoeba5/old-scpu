mod assembler;

use clap::Parser;
use anyhow::{Result, Context};
use assembler::module::Module;

#[derive(Parser, Debug)]
struct Args {
    #[clap()]
    input_file: String,

    #[clap(short = 'o', long = "output", default_value_t=String::from("a.out"))]
    output_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let source = std::fs::read_to_string(&args.input_file).with_context(|| format!("Unable to open file '{}' for reading", args.input_file))?;
   
    let mut module = Module::new();
    module.preprocess(source)?;

    Ok(())
}
