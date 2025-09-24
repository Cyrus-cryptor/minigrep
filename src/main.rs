use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let conf = parse_conf(&args);

    println!("The query is: {}",conf.query);
    println!("The file path is: {}",conf.file_path);

    let contents = fs::read_to_string(conf.file_path)
        .expect("should been able to read this file");

    println!("with test:\n{contents}");

}

struct Config {
    query: String,
    file_path: String,
}

fn parse_conf(args: &[String]) -> Config {

    if args.len() <= 2 {
        panic!("No enough args....")
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {
        query: query,
        file_path: file_path,
    }
}
