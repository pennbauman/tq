use std::fmt;
use toml::Value;

#[derive(Debug)]
enum TomlKey {
    Table(String),
    Array(usize),
}

#[derive(Debug)]
enum ParseState {
    Start,
    EmptyKey,
    PartialKey,
    EmptyIndex,
    PartialIndex,
}

#[derive(Debug)]
pub struct KeyPattern {
    keys: Vec<TomlKey>,
}
impl KeyPattern {
    // Parse key sequence from string
    pub fn parse(source: &str) -> Result<Self, String> {
        if source == "." {
            return Ok(Self { keys: vec![] });
        }
        let mut keys = vec![];
        //let mut i = 0;
        let mut state = ParseState::Start;
        let mut current = String::new();
        for c in source.chars() {
            match state {
                ParseState::Start => {
                    if c == '.' {
                        state = ParseState::EmptyKey;
                    } else if c == '[' {
                        state = ParseState::EmptyIndex;
                    } else if c == ']' {
                        return Err(format!("Invalid Key: Missing opening bracket"));
                    } else {
                        state = ParseState::PartialKey;
                        current.push(c);
                    }
                },
                ParseState::EmptyKey => {
                    if c == '.' {
                        return Err(format!("Invalid Key: Consecutive dots '..'"));
                    } else if c == '[' {
                        state = ParseState::EmptyIndex;
                    } else if c == ']' {
                        return Err(format!("Invalid Key: Missing opening bracket"));
                    } else {
                        state = ParseState::PartialKey;
                        current.push(c);
                    }
                },
                ParseState::PartialKey => {
                    if c == '.' {
                        keys.push(TomlKey::Table(current));
                        current = String::new();
                        state = ParseState::EmptyKey;
                    } else if c == '[' {
                        keys.push(TomlKey::Table(current));
                        current = String::new();
                        state = ParseState::EmptyIndex;
                    } else if c == ']' {
                        return Err(format!("Invalid Key: Missing opening bracket"));
                    } else {
                        current.push(c);
                    }
                },
                ParseState::EmptyIndex => {
                    if c == '.' {
                        return Err(format!("Invalid Key: Dot inside brackets"));
                    } else if c == '[' {
                        return Err(format!("Invalid Key: Nested brackets"));
                    } else if c == ']' {
                        return Err(format!("Invalid Key: Empty brackets"));
                    } else {
                        state = ParseState::PartialIndex;
                        current.push(c);
                    }
                },
                ParseState::PartialIndex => {
                    if c == '.' {
                        return Err(format!("Invalid Key: Dot inside brackets"));
                    } else if c == '[' {
                        return Err(format!("Invalid Key: Nested brackets"));
                    } else if c == ']' {
                        let n: usize = match current.parse() {
                            Ok(n) => n,
                            Err(_) => return Err(format!("Invalid Key: Invalid number '{}'", current)),
                        };
                        keys.push(TomlKey::Array(n));
                        current = String::new();
                        state = ParseState::EmptyKey;
                    } else {
                        current.push(c);
                    }
                },
            }
            print!("{}", c);
        }
        match state {
            ParseState::Start => return Err(format!("Invalid Key: Blank")),
            ParseState::EmptyKey => return Err(format!("Invalid Key: Ends with dot")),
            ParseState::PartialKey => keys.push(TomlKey::Table(current)),
            ParseState::EmptyIndex => return Err(format!("Invalid Key: Unclosed bracket")),
            ParseState::PartialIndex => return Err(format!("Invalid Key: Unclosed bracket")),
        }
        println!("{:?}", keys);

        Ok(Self { keys: keys })
    }
    // Get value based on key sequence
    pub fn find<'a>(&self, toml: &'a Value) -> Result<&'a Value, String> {
        let mut current = toml;
        for k in &self.keys {
            match k {
                TomlKey::Table(s) => {
                    match current {
                        toml::Value::Table(t) => {
                            current = match t.get(s) {
                                Some(v) => v,
                                None => return Err(s.to_string()),
                            };
                        },
                        _ => {
                            return Err(s.to_string());
                        }
                    }
                },
                TomlKey::Array(n) => {
                    match current {
                        toml::Value::Array(a) => {
                            if a.len() < *n {
                                return Err(n.to_string())
                            }
                            current = &a[*n];
                        },
                        _ => {
                            return Err(n.to_string());
                        }
                    }
                },
            }
        }
        Ok(current)
    }
}
// Print
impl fmt::Display for KeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for k in &self.keys {
            match k {
                TomlKey::Table(s) => write!(f, ".{}", s)?,
                TomlKey::Array(n) => write!(f, ".[{}]", n)?,
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_key() {
        let key = KeyPattern::parse(".").unwrap();
        assert!(key.keys.len() == 0);
    }
}
