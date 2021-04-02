#[link(name="yes", vers="1.0.0", author="Xue Jiao")]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        exec("y");
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

fn exec(s: &str) {
    loop {
        println!("{}", s);
    }
}

fn help() {
    println!("yes 1.0.0
Xue Jiao <jiao.xuejiao@gmail.com>
Repeatedly output a line with all specified STRING, or 'y'.

Usage:
    yes [STRING]...

FLAGS:
    -h, --help       display this help and exit
    -V, --version    output version information and exit
");
}

fn version() {
    println!("yes 1.0.0");
}
