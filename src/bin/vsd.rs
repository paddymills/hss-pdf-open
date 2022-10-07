use regex::Regex;
use std::{io, path::PathBuf};
use clap::Parser;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Parser)]
#[command(author, version, name = "View Shop Drawings", about = "Shop Drawing file launcher")]
struct Cli {
    #[arg(value_parser = parse_job)]
    job: String,

    #[arg(value_parser = parse_dwg)]
    dwgs: Vec<Vec<String>>,
}

impl Cli {
    fn open_files(self) -> io::Result<()> {
        let job_root = PathBuf::from(r"\\Hssieng\plp\shopdwgs").join(self.job);

        for dwg in self.dwgs.into_iter().flatten() {
            let root = job_root.join(&dwg).with_extension("PDF");
            let prelim = job_root
                .join("Preliminary")
                .join(&dwg)
                .with_extension("PDF");

            if root.exists() {
                if let Ok(_) = opener::open(root) {
                    println!("{} from root(approved & released)", &dwg);
                }
            } else if prelim.exists() {
                if let Ok(_) = opener::open(prelim) {
                    println!("{} from preliminary", &dwg);
                }
            } else {
                println!("{} not found", &dwg)
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    // TODO: launch ViewShopDrawings if no args
    // PathBuf::from(r"\\HSSIENG\Resources\HS\PROG\DCS\ViewShopDrawingsNET.exe")
    let _ = args.open_files();

    Ok(())
}

fn parse_job(job: &str) -> Result<String, String> {
    Ok(String::from(job.to_uppercase()))
}

fn parse_dwg(dwg: &str) -> Result<Vec<String>, String> {
    let dwgs = match dwg.split('-').collect::<Vec<&str>>()[..] {
        [_, _] => {
            lazy_static! {
                static ref PATTERN: Regex = Regex::new(r"([a-zA-Z]*)([0-9]+)-[a-zA-Z]*([0-9]+)").unwrap();
            }

            match PATTERN.captures(dwg) {
                Some(caps) => {
                    let mut dwgs = vec![];
                    let prefix = &caps[1];
                    let (a, b) = (&caps[2], &caps[3]);

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
                        dwgs.push(String::from(format!("{}{}", prefix, i)).to_uppercase());
                    }

                    dwgs
                }
                None => vec![String::from(dwg)],
            }
        }
        _ => vec![String::from(dwg)],
    };

    Ok(dwgs)
}
