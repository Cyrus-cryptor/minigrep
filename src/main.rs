use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    // let conf  =  parse_conf(&args);
    //let conf =  Config::new(&args);
    let conf = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("err is {err}, and exit with code 101");
            process::exit(101)
        });


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

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() <=2 {
    //         panic!("error to ceate Config since no enough args")
    //     }
    //     Config { 
    //         query: args[1].clone(),
    //         file_path: args[2].clone() 
    //     }
    // }
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            Err("no enouht args")
        } else {
            Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
        }
    }
}

// fn parse_conf(args: &[String]) -> Config {

//     if args.len() <= 2 {
//         panic!("No enough args....")
//     }
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config {
//         query: query,
//         file_path: file_path,
//     }
// }
