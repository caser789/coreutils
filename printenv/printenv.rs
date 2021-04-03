#[link(name="printenv", vers="1.0.0", author="Xue Jiao")]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_all();
        return
    }

    let option = &args[1];
    match &option[..] {
        "-h" => help(),
        "--help" => help(),
        "-V" => version(),
        "--version" => version(),
        _ => {
            exec(option);
        }
    }
}

fn exec(key: &str) {
    match env::var(key) {
        Ok(val) => println!("{}", val),
        Err(_) => {},
    }
}

fn help() {
    println!("printenv 1.0.0
Xue Jiao <jiao.xuejiao@gmail.com>
Prints the given environment VARIABLE, otherwise prints them all.

Usage:
    printenv [STRING]...

FLAGS:
    -h, --help       display this help and exit
    -V, --version    output version information and exit
");
}

fn version() {
    println!("printenv 1.0.0");
}

fn print_all() {
    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }
}
