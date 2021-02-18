#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    #[test]
    fn test_skip_next() {
        assert_eq!(skip_next("(foo", '('), Some("foo"));
        assert_eq!(skip_next("(foo", '*'), None);
        assert_eq!(skip_next("ѝfoo", 'ѝ'), Some("foo"));
        assert_eq!(skip_next("ѝ😊ѝ😊", 'ѝ'), Some("😊ѝ😊"));
        assert_eq!(skip_next("", '('), None);
    }
    #[test]
    fn test_take_until() {
        assert_eq!(take_until(" foo/bar ", '/'), (" foo", "/bar "));
        assert_eq!(take_until(" foo/bar/foo// ", '/'), (" foo", "/bar/foo// "));
        assert_eq!(take_until(" foo/bar ѝ", 'ѝ'), (" foo/bar ", "ѝ"));
        assert_eq!(take_until("foobar", '/'), ("foobar", ""));
        assert_eq!(take_until("", '/'), ("", ""));
    }
    #[test]
    fn test_take_and_skip() {
        assert_eq!(take_and_skip(" foo/bar ", '/'), Some((" foo", "bar ")));
        assert_eq!(
            take_and_skip(" foo/bar/foo// ", '/'),
            Some((" foo", "bar/foo// "))
        );
        assert_eq!(take_and_skip(" foo/bar ѝ", 'ѝ'), Some((" foo/bar ", "")));
        assert_eq!(take_and_skip("foobar", '/'), None);
        assert_eq!(take_and_skip("", '/'), None);
    }

    use std::io::{self, Read};

    macro_rules! assert_match {
        ($expr:expr, $pat:pat) => {
            if let $pat = $expr {
                // all good
            } else {
                assert!(
                    false,
                    "Expression {:?} does not match the pattern {:?}",
                    $expr,
                    stringify!($pat)
                );
            }
        };
    }

    struct ErroringReader {}
    impl Read for ErroringReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "read error!"))
        }
    }

    impl BufRead for ErroringReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(io::Error::new(io::ErrorKind::Other, "fill_buf error!"))
        }

        fn consume(&mut self, _amt: usize) {}
    }

    fn make_csv<'a>(data: &'a str) -> Result<Csv<&'a [u8]>, CsvError> {
        Csv::new(data.as_bytes())
    }
    #[test]
    fn test_csv_new_errors() {
        assert_match!(Csv::new(ErroringReader {}).err(), Some(CsvError::IO(_)));
        assert_match!(make_csv("").err(), Some(CsvError::InvalidHeader(_)));
        assert_match!(make_csv("\n").err(), Some(CsvError::InvalidHeader(_)));
        assert_match!(make_csv("djskd, dsjk   , ,\n").err(), None);
        assert_match!(
            make_csv("djskd, dsjk   , ,    , djk\n").err(),
            Some(CsvError::InvalidHeader(_))
        );
        assert_match!(make_csv("djskaljsd").err(), None);
        assert_match!(
            make_csv("dj, dja, dsjask, dj\n").err(),
            Some(CsvError::InvalidHeader(_))
        );
        assert_match!(make_csv("djskaljsd\n").err(), None);
        assert_match!(make_csv("djskaljsd, jkjdskd,  \n").err(), None);
    }
    #[test]
    fn test_parse_line_errors() {
        let row = make_csv("name, age, birth date \n")
            .unwrap()
            .parse_line(r#""Basic Name","13","2020-01-01""#)
            .unwrap();
        assert_eq! {
            (row["name"].as_str(), row["age"].as_str(), row["birth date"].as_str()),
            ("Basic Name", "13", "2020-01-01"),
        };
        let row2 = make_csv("name, age, birth date \n")
            .unwrap()
            .parse_line(r#""Basic,Name"," 13 ","2020,01,01""#)
            .unwrap();
        assert_eq! {
            (row2["name"].as_str(), row2["age"].as_str(), row2["birth date"].as_str()),
            ("Basic,Name", " 13 ", "2020,01,01"),
        };
        assert_match!(
            make_csv("a, b\n")
                .unwrap()
                .parse_line(r#" "jkjk "jkjk" "#)
                .err(),
            Some(CsvError::InvalidRow(_))
        );
        assert_match!(
            make_csv("a, b\n")
                .unwrap()
                .parse_line(r#" "jkjk" , "jkjk "#)
                .err(),
            Some(CsvError::InvalidRow(_))
        );
        assert_match!(
            make_csv("a, b\n").unwrap().parse_line(r#" "jkjk", "#).err(),
            Some(CsvError::InvalidRow(_))
        );
        assert_match!(
            make_csv("a, b\n")
                .unwrap()
                .parse_line(r#" "jkjk", "a", "b" "#)
                .err(),
            Some(CsvError::InvalidRow(_))
        );
        assert_match!(
            make_csv("a, b\n")
                .unwrap()
                .parse_line(r#" "jkjk" , "jkjk"    "#)
                .err(),
            None
        );
        assert_match!(
            make_csv("a, b\n")
                .unwrap()
                .parse_line(r#" "jk , jk" , "jk jk",    "#)
                .err(),
            None
        );
        assert_match!(
            make_csv("a, b, c\n")
                .unwrap()
                .parse_line(r#" "jk , jk" ,"", "jk jk", "#)
                .err(),
            None
        );
    }
}

pub fn skip_next(input: &str, target: char) -> Option<&str> {
    match input.char_indices().next() {
        Some((0, c)) => {
            if c != target {
                return None;
            } else {
                return Some(input.split_at(c.len_utf8()).1);
            }
        }
        _ => return None,
    }
}

pub fn take_until(input: &str, target: char) -> (&str, &str) {
    let mut iter = input.char_indices();
    while let Some((pos, c)) = iter.next() {
        if c == target {
            return input.split_at(pos);
        }
    }
    return (input, "");
}

pub fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
    let (l, r) = take_until(input, target);
    match skip_next(r, target) {
        Some(r) => Some((l, r)),
        _ => None,
    }
}

#[derive(Debug)]
pub enum CsvError {
    IO(std::io::Error),
    //ParseError(String),
    InvalidHeader(String),
    InvalidRow(String),
    //InvalidColumn(String),
}

use std::collections::{HashSet, HashMap};

type Row = HashMap<String, String>;

use std::io::BufRead;

pub struct Csv<R: BufRead> {
    pub columns: Vec<String>,
    reader: R,
    selection: Option<Box<dyn Fn(&Row) -> Result<bool, CsvError>>>,
}

impl<R: BufRead> Csv<R> {
    pub fn new(mut reader: R) -> Result<Self, CsvError> {
        let mut header: HashSet<String> = HashSet::new();
        let mut line: String = String::new();
        match reader.read_line(&mut line) {
            Err(e) => Err(CsvError::IO(e)),
            Ok(0) => Err(CsvError::InvalidHeader(String::from("Empty reader"))),
            _ => {
                let mut columns: Vec<String> = Vec::new();
                line = line.trim().to_string();
                while line.len() != 0 {
                    let (mut column, rem) = take_until(&line, ',');
                    column = column.trim();
                    if header.contains(column) == true {
                        return Err(CsvError::InvalidHeader(String::from(
                            "Duplicate column names!",
                        )));
                    }
                    
                    header.insert(column.to_string());
                    columns.push(column.to_string());
                    match skip_next(&rem, ',') {
                        Some(s) => line = s.to_string(),
                        _ => break,
                    }
                }

                if columns.len() == 0 {
                    return Err(CsvError::InvalidHeader(String::from("No columns!")));
                }
                return Ok(Self {
                    columns,
                    reader,
                    selection: None,
                });
            }
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<Row, CsvError> {
        let mut row: Row = Row::new();
        let n = self.columns.len();
        let mut rem = line;
        for i in 0..n {
            rem = rem.trim();
            if rem.len() == 0 {
                return Err(CsvError::InvalidRow(String::from(
                    "Smaller number of values!",
                )));
            }
            match skip_next(&rem, '\"') {
                Some(s) => rem = s,
                _ => {
                    match take_and_skip(rem, ',') {
                        Some((data, s)) => {
                            if s.len() > 0 && i + 1 == n {
                                println!("{}", s);
                                return Err(CsvError::InvalidRow(String::from("Text after last data!")));
                            }
                            rem = s;
                            row.insert(self.columns[i].clone(), data.to_string());
                        }
                        _ => {
                            if i + 1 < n {
                                return Err(CsvError::InvalidRow(String::from("No comma between data!")));
                            }
                        }
                    }
                    continue;
                },
            }
            match take_and_skip(rem, '\"') {
                Some((data, s)) => {
                    rem = s;
                    row.insert(self.columns[i].clone(), data.to_string());
                }
                _ => return Err(CsvError::InvalidRow(String::from("No closing \" !"))),
            }
            match take_and_skip(rem, ',') {
                Some((_, s)) => {
                    if s.len() > 0 && i + 1 == n {
                        return Err(CsvError::InvalidRow(String::from("Text after last data!")));
                    }
                    rem = s;
                }
                _ => {
                    if i + 1 < n {
                        return Err(CsvError::InvalidRow(String::from("No comma between data!")));
                    }
                }
            }
        }
        return Ok(row);
    }
}

impl<R: BufRead> Iterator for Csv<R> {
    type Item = Result<Row, CsvError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Err(e) => return Some(Err(CsvError::IO(e))),
            Ok(0) => return None,
            _ => match self.parse_line(&line) {
                Err(e) => return Some(Err(e)),
                Ok(row) => match self.selection {
                    Some(_) => match self.selection.as_ref().unwrap()(&row) {
                        Err(e) => return Some(Err(e)),
                        Ok(true) => return Some(Ok(row)),
                        Ok(false) => return self.next(),
                    },
                    None => return Some(Ok(row)),
                },
            },
        }
    }
}
