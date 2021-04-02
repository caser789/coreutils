#[link(name="yes", vers="1.0.0", author="Xue Jiao")]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        exec("y");
    }

    let cmd = &args[0];
    let option = &args[1];
    match &option[..] {
        "-h" => help(cmd),
        "--help" => help(cmd),
        "-V" => version(),
        "--version" => version(),
        _ => {
            exec(option);
        }
    }

    println!("{:?}", args);

}

fn exec(s: &str) {
    loop {
        println!("{}", s);
    }
}

fn help(s: &str) {
    println!("yes 1.0.0
Usage:
    {} [STRING]... [OPTION]...

Options:
    -h --help display this help and exit
    -V --version output version information and exit

Repeatedly output a line with all specified STRING, or 'y'.
", s);
}

fn version() {
    println!("yes 1.0.0");
}
