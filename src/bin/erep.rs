use clap::Parser;
use regex::Regex;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    name = "View Shop Drawings",
    about = "Shop Drawing file launcher"
)]
struct Cli {
    #[arg(value_parser = parse_prog)]
    progs: Vec<Vec<String>>,
}

impl Cli {
    fn open_files(self) {
        const PROG_LEN: usize = 5; // must be usize for len comparison and trucate
        let mut last_prog = String::from("00000");

        let erep_root = PathBuf::from(r"\\hssfileserv1\Shops\eReports");

        for prog in self.progs.into_iter().flatten() {
            let prog = match prog.parse::<u32>() {
                // is number
                Ok(_) => match PROG_LEN - prog.len() {
                    x if x > 0 => vec![&last_prog[0..x], &prog].concat(),
                    _ => prog,
                },
                Err(_) => prog, // has non-numeric characters
            };

            let root = erep_root.join(&prog).with_extension("PDF");

            if root.exists() {
                if let Ok(_) = opener::open(root) {
                    println!("Opening: {}", &prog);
                }
            } else {
                println!("{} not found", &prog)
            }

            last_prog = prog;
            if last_prog.len() > PROG_LEN {
                last_prog.truncate(PROG_LEN);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    args.open_files();

    Ok(())
}

fn parse_prog(prog: &str) -> Result<Vec<String>, String> {
    let ereps = match prog.split('-').collect::<Vec<&str>>()[..] {
        [_, _] => {
            lazy_static! {
                static ref PATTERN: Regex = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
            }

            match PATTERN.captures(prog) {
                Some(caps) => {
                    let mut progs = vec![];
                    let (a, b) = (&caps[1], &caps[2]);

                    // regex should ensure these two parse numerically and
                    // conventionally these numbers should never exceed u32::MAX
                    // therefore ->    should not panic!
                    let start: u32 = a.parse().unwrap();
                    let end: u32 = match a.len() - b.len() {
                        x if x > 0 => {
                            let c = vec![&a[0..x], &b].concat();
                            c.parse().unwrap()
                        }
                        _ => b.parse().unwrap(), // same length or b is longer (i.e. 1-20)
                    };

                    for i in start..end + 1 {
                        progs.push(i.to_string());
                    }

                    progs
                }
                None => vec![String::from(prog)],
            }
        }
        _ => vec![String::from(prog)],
    };

    Ok(ereps)
}
