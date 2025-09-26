use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};
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
    if let Err(e) = run(conf) {
        println!("Application error: {e}");
        process::exit(1)
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
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
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config { query: args[1].clone(), file_path: args[2].clone(), ignore_case: ignore_case, })
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

// fn run(conf: Config) {
//     let contents  = fs::read_to_string(conf.file_path)
//         .expect("Shouble be able to read file");
//     println!("the contents is {contents}");
// }

fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents  = fs::read_to_string(conf.file_path)?;
    
    for line in if conf.ignore_case {
        search_case_insensitive(&conf.query, &contents)
    } else {
        search(&conf.query, &contents)
    } {
        println!("{line}")
    };
    Ok(())
}