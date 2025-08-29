use clap::Parser;
use regex::Regex;
use std::{num::ParseIntError, path::PathBuf};

#[macro_use]
extern crate lazy_static;

fn u32_len(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => (n as f64).log10().floor() as u32 + 1,
    }
}

fn fix_len(n: u32, prev: u32) -> u32 {
    match [n, prev].map(u32_len) {
        [n_len, prev_len] if n_len < prev_len => {
            // Calculate how many leading digits we need from prev
            let digits_needed = prev_len - n_len;

            // Extract the leading digits from prev
            let divisor = 10_u32.pow(prev_len - digits_needed);
            let leading_part = prev / divisor;

            // Combine: leading_part followed by n
            let multiplier = 10_u32.pow(n_len);
            leading_part * multiplier + n
        }
        _ => n,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CliProg {
    Single(u32),
    Range(u32, u32),
}

impl CliProg {
    fn fix_len(&self, prev: Option<u32>) -> Self {
        match (self, prev) {
            (_, None) => self.clone(),
            (CliProg::Single(n), Some(prev)) => CliProg::Single(fix_len(*n, prev)),
            (CliProg::Range(a, b), Some(prev)) => {
                let new_a = fix_len(*a, prev);
                CliProg::Range(new_a, fix_len(*b, new_a))
            }
        }
    }
}

impl IntoIterator for CliProg {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<u32>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CliProg::Single(n) => vec![n].into_iter(),
            CliProg::Range(a, b) => (a..=b).collect::<Vec<u32>>().into_iter(),
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    name = "eReports Launcher",
    about = "eReports file launcher"
)]
struct Cli {
    #[arg(value_parser = parse_prog)]
    progs: Vec<CliProg>,
}

impl Cli {
    fn open_files(self) {
        let mut last_prog: Option<u32> = None;

        let erep_root = PathBuf::from(r"\\hssfileserv1\Shops\eReports");

        for prog in self.progs {
            println!("Processing: {:?}", &prog);
            last_prog = prog
                .fix_len(last_prog)
                .into_iter()
                .map(|full_prog| {
                    println!(" -> Searching for: {}", &full_prog);

                    let root = erep_root.join(full_prog.to_string()).with_extension("PDF");
                    if root.exists() {
                        if let Ok(_) = opener::open(root) {
                            println!("✅Opening: {}", &full_prog);
                        }
                    } else {
                        println!("❌{} not found", &full_prog)
                    }

                    full_prog
                })
                .last();
        }
    }
}

fn main() -> Result<(), String> {
    Cli::parse().open_files();

    Ok(())
}

fn parse_prog(prog: &str) -> Result<CliProg, ParseIntError> {
    let ereps = match prog.split('-').collect::<Vec<&str>>()[..] {
        [_, _] => {
            lazy_static! {
                static ref PATTERN: Regex = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
            }

            match PATTERN.captures(prog) {
                Some(caps) => CliProg::Range(caps[1].parse()?, caps[2].parse()?),
                None => CliProg::Single(prog.parse()?),
            }
        }
        _ => CliProg::Single(prog.parse()?),
    };

    Ok(ereps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_len() {
        assert_eq!(u32_len(0), 1);
        assert_eq!(u32_len(5), 1);
        assert_eq!(u32_len(10), 2);
        assert_eq!(u32_len(99), 2);
        assert_eq!(u32_len(100), 3);
        assert_eq!(u32_len(999), 3);
        assert_eq!(u32_len(1000), 4);
        assert_eq!(u32_len(12345), 5);
    }

    #[test]
    fn test_fix_len_same_length() {
        assert_eq!(fix_len(123, 456), 123);
        assert_eq!(fix_len(99, 88), 99);
    }

    #[test]
    fn test_fix_len_shorter_number() {
        assert_eq!(fix_len(23, 1234), 1223);
        assert_eq!(fix_len(5, 123), 125);
        assert_eq!(fix_len(67, 1234), 1267);
        assert_eq!(fix_len(1, 9876), 9871);
    }

    #[test]
    fn test_fix_len_longer_number() {
        assert_eq!(fix_len(1234, 56), 1234);
        assert_eq!(fix_len(999, 12), 999);
    }

    #[test]
    fn test_cliprog_single() {
        let prog = CliProg::Single(123);
        let items: Vec<u32> = prog.into_iter().collect();
        assert_eq!(items, vec![123]);
    }

    #[test]
    fn test_cliprog_range() {
        let prog = CliProg::Range(5, 8);
        let items: Vec<u32> = prog.into_iter().collect();
        assert_eq!(items, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_cliprog_fix_len_single_no_prev() {
        let prog = CliProg::Single(123);
        let fixed = prog.fix_len(None);
        assert_eq!(fixed, CliProg::Single(123));
    }

    #[test]
    fn test_cliprog_fix_len_single_with_prev() {
        let prog = CliProg::Single(23);
        let fixed = prog.fix_len(Some(1234));
        assert_eq!(fixed, CliProg::Single(1223));
    }

    #[test]
    fn test_cliprog_fix_len_range_no_prev() {
        let prog = CliProg::Range(5, 8);
        let fixed = prog.fix_len(None);
        assert_eq!(fixed, CliProg::Range(5, 8));
    }

    #[test]
    fn test_cliprog_fix_len_range_with_prev() {
        let prog = CliProg::Range(23, 25);
        let fixed = prog.fix_len(Some(1234));
        assert_eq!(fixed, CliProg::Range(1223, 1225));
    }

    #[test]
    fn test_parse_prog_single() {
        let result = parse_prog("123").unwrap();
        assert_eq!(result, CliProg::Single(123));
    }

    #[test]
    fn test_parse_prog_range() {
        let result = parse_prog("123-456").unwrap();
        assert_eq!(result, CliProg::Range(123, 456));
    }

    #[test]
    fn test_parse_prog_invalid_number() {
        assert!(parse_prog("abc").is_err());
    }

    #[test]
    fn test_parse_prog_range_with_invalid_number() {
        assert!(parse_prog("123-abc").is_err());
    }

    #[test]
    fn test_parse_prog_hyphen_but_not_range() {
        assert!(parse_prog("123-456-789").is_err());
    }
}
