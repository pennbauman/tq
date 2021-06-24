use std::fs;
use std::io::Read;

mod key;

// Command line arguments
#[derive(Debug)]
struct Args {
    key: String,
    file: Option<String>,
}
impl Args {
    fn parse() -> Result<Self, String> {
        let mut pargs = pico_args::Arguments::from_env();
        let args = Self {
            key: match pargs.free_from_str() {
                Ok(s) => s,
                Err(_) => return Err(String::from("Arguments Err: Missing key string")),
            },
            file: match pargs.opt_free_from_str() {
                Ok(s) => s,
                Err(_) => None,
            }
        };
        let remaining = pargs.finish();
        if !remaining.is_empty() {
            return Err(format!("Unused argument {:?}", remaining[0]));
        }
        Ok(args)
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = match Args::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Parse toml to use
    let toml: toml::Value = match args.file {
        Some(s) => {
            let mut file = fs::File::open(s)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            match toml::from_str(&contents) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("TOML Error: {}", e);
                    std::process::exit(1);
                },
            }
        },
        None => {
            //let stdin = io::stdin();
            //for line in stdin.lock().lines() {
                //let line = line.expect("Could not read line from standard in");
                //println!("{}", line);
            //}
            eprintln!("Error: No toml to parse");
            std::process::exit(1);
        },
    };

    // Parse key arguements
    let keys = match key::KeyPattern::parse(&args.key) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Invalid key '{}'", e);
            std::process::exit(1);
        }
    };
    // Get value to print
    let current = match keys.find(&toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Unknown key '{}'", e);
            std::process::exit(1);
        }
    };

    // Print results
    let output = match current {
        toml::Value::Table(t) => match toml::to_string_pretty(t) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Formatting: {}", e);
                std::process::exit(1);
            },
        },
        toml::Value::Array(a) => match toml::to_string_pretty(a) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Formatting: {}", e);
                std::process::exit(1);
            },
        },
        toml::Value::String(s) => s.to_string(),
        x => x.to_string(),
    };
    println!("{}", output);
    Ok(())
}
