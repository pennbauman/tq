use std::fmt;
use toml::Value;

enum TomlKey {
    Table(String),
    Array(usize),
}

pub struct KeyPattern {
    keys: Vec<TomlKey>,
}
impl KeyPattern {
    // Parse key sequence from string
    pub fn parse(source: &str) -> Result<Self, &str> {
        let mut keys = vec![];
        let mut i = 0;
        for s in source.split(".") {
            if s == "" {
                if i != 0 {
                    return Err("..");
                }
                i -= 1;
            } else {
                keys.push(TomlKey::Table(s.to_string()));
            }
            i += 1;
        }

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
                TomlKey::Array(n) => write!(f, "[{}]", n)?,
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
