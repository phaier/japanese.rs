#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    FailedToDecode,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedToDecode => write!(f, "Failed to decode the CSV data"),
        }
    }
}
impl std::error::Error for Error {}

pub fn parse_csv(content: &str) -> Result<Vec<Vec<String>>, Error> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut column: Vec<char> = Vec::new();
    let mut chars = content.chars();

    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                if column.len() > 0 {
                    row.push(String::from_iter(column));
                    column = Vec::new();
                } else {
                    row.push(String::new());
                }

                rows.push(row);
                row = Vec::new();
            }
            ',' => {
                row.push(String::from_iter(column));
                column = Vec::new();
            }
            '"' => {
                let mut quoted: Vec<char> = vec![];

                loop {
                    match chars.next() {
                        Some('"') => {
                            break;
                        }
                        Some('\\') => {
                            if let Some(next) = chars.next() {
                                quoted.push(next);
                            }
                        }
                        Some(c) => quoted.push(c),
                        None => break,
                    }
                }

                row.push(String::from_iter(quoted));
                column = Vec::new();

                if let Some(c) = chars.clone().peekable().peek() {
                    if *c == ',' {
                        chars.next();
                    }
                }
            }
            _ => {
                column.push(c);
            }
        }
    }

    if column.len() > 0 {
        row.push(String::from_iter(column.iter()));
    }

    if row.len() > 0 {
        rows.push(row);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let dataset: Vec<(&str, Vec<Vec<&str>>)> = vec![
            (
                "name,age,city\nAlice,30,Tokyo\nBob,25,Osaka",
                vec![
                    vec!["name", "age", "city"],
                    vec!["Alice", "30", "Tokyo"],
                    vec!["Bob", "25", "Osaka"],
                ],
            ),
            (
                "Alice,30,\"Tokyo, Japan\",40",
                vec![vec!["Alice", "30", "Tokyo, Japan", "40"]],
            ),
            (
                "Alice,30,\"Tokyo\nJapan\",40",
                vec![vec!["Alice", "30", "Tokyo\nJapan", "40"]],
            ),
        ];

        for (input, expected) in dataset {
            let result = parse_csv(input).unwrap();
            assert_eq!(result, expected, "Failed to parse CSV: {}", input);
        }
    }
}
