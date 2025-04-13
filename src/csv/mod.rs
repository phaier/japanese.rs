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
    let mut column = Vec::new();
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
        let content = "name,age,city\nAlice,30,Tokyo\nBob,25,Osaka";
        let result = parse_csv(content).unwrap();

        assert_eq!(
            result,
            vec![
                vec!["name", "age", "city"],
                vec!["Alice", "30", "Tokyo"],
                vec!["Bob", "25", "Osaka"],
            ]
        );
    }
}
