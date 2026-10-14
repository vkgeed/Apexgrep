use apexgrep::Config;
use apexgrep::{config_args, run, search};
use std::{env, println};

fn main() {
    // args = [String::from("/apexgrep/target/debug/apexgrep.exe"), String::from("apex"), String::from("example.txt")]
    let args: Vec<String> = env::args().collect();

    //t1` :  extract the args
    let config: Result<Config, &'static str> = config_args(&args);

    let config_result = match config {
        Ok(config) => config,
        Err(e) => {
            eprintln!(" Error : {}", e);
            panic!("Error : {}", e);
            return 
        }
    };

    // t2 : read the file
    run(config_result);
}

// we can decalate the helper function here or we can create a module and define there !
// for this case and to make things interesting let's make another file and call from cofig fn from there !
