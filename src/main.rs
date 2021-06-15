use std::fs;
use std::{io, io::{Read, BufReader, BufRead}};

// Command line arguments
#[derive(Debug)]
struct Args {
    key: String,
    file: Option<String>,
}
impl Args {
    fn parse() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        let args = Self {
            key: pargs.opt_free_from_str()?.unwrap_or(String::new()),
            file: pargs.opt_free_from_str()?,
        };
        let remaining = pargs.finish();
        if !remaining.is_empty() {
            eprintln!("Unused argument {:?}", remaining[0]);
            std::process::exit(1);
        }
        Ok(args)
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = match Args::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Arguments Error: {}.", e);
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
            let mut buf = BufReader::new(io::stdin());
            let mut contents = String::new();
            buf.fill_buf()?;
            buf.read_to_string(&mut contents)?;
            toml::from_str(&contents)?
        },
    };

    // Fold key to get requested values
    let mut i = 0;
    let mut current = &toml;
    for s in args.key.split(".") {
        if s == "" {
            if i != 0 {
                eprintln!("Invalid key '..'");
                std::process::exit(1);
            }
            i -= 1;
        } else {
            current = match current.get(s) {
                Some(v) => v,
                None => {
                    eprintln!("Invalid key '{}'", s);
                    std::process::exit(1);
                },
            };
        }
        i += 1;
    }

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
