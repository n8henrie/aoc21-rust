use std::path::Path;

pub fn localpath(path: impl AsRef<Path>) -> anyhow::Result<impl AsRef<Path>> {
    let envvar = std::env::var("CARGO_MANIFEST_DIR")?;
    let basedir = Path::new(&envvar);
    Ok(basedir.join(path))
}

#[macro_export]
macro_rules! parse_input {
    ($path:expr) => {
        parse_input!($path, String)
    };
    ($path:expr, $ty:ty) => {{
        use anyhow::{anyhow, Context};
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        File::open($path)
            .context("unable to open file")
            .and_then(|f| {
                BufReader::new(f)
                    .lines()
                    .map(|bufline| {
                        bufline
                            .context("error iterating over bufreader")
                            .and_then(|line| {
                                line.parse::<$ty>()
                                    .context("Unable to parse as type")
                            })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .map_err(|err| anyhow!(err))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_localpath() {
        let path = localpath("foo.txt").unwrap();
        let path = path.as_ref().to_str().unwrap();
        assert!(path.ends_with("aoc21-rust/aoc/foo.txt"));
    }

    #[test]
    fn test_read_to_u32() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "123\n456").unwrap();
        let expected = vec![123_u32, 456];
        let result = parse_input!(tmpfile.path(), u32);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_read_to_string() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "123\n456").unwrap();
        let expected: Vec<String> =
            ["123", "456"].into_iter().map(Into::into).collect();
        let result = parse_input!(tmpfile.path()).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_read_to_custom_type() {
        use anyhow::bail;

        #[derive(Debug, PartialEq)]
        struct Point {
            x: u32,
            y: u32,
        }

        impl std::str::FromStr for Point {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut splitter = s.split_whitespace();
                match (splitter.next(), splitter.next()) {
                    (Some(x), Some(y)) => Ok(Point {
                        x: x.parse()?,
                        y: y.parse()?,
                    }),
                    _ => bail!("Unable to parse"),
                }
            }
        }

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "1 2\n3 4").unwrap();
        let expected = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
        let result = parse_input!(tmpfile.path(), Point).unwrap();
        assert_eq!(expected, result);
    }
}
