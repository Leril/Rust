use std::time::Duration;
use clap::Parser;
use crate::interpreter::VM;

mod interpreter;

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let filename:  &str = &cli.rom;

    let speed = Duration::from_secs_f64((1 / 700) as f64);
    let vm = VM::new(speed)
        .load(filename)
        .unwrap_or_else(|_| panic!("Could not load ROM: {}", filename));

    chip8_base::run(vm);
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli{
    //#[clap(validator = check_if_exists)]
    rom: String
}

fn check_if_exists(file: &str) -> Result<(), &'static str>{
    let p = std::path::Path::new(file);

    if !p.is_file(){
        Err("file does not exist")
    }else{
        Ok(())
    }
}
