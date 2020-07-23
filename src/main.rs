use exitfailure::ExitFailure;
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, StructOpt)]
#[structopt(name = "View Shop Drawings", about = "Shop Drawing file launcher")]
struct Cli {
    #[structopt(parse(from_str = parse_job))]
    job: String,

    #[structopt(parse(from_str = parse_dwg))]
    dwgs: Vec<Vec<String>>,
}

impl Cli {
    fn open_files(self) {
        let job_root = PathBuf::from(r"\\Hssieng\plp\shopdwgs").join(self.job);

        let mut args: Vec<String> = Vec::new();
        for dwg in self.dwgs.into_iter().flatten() {
            let root = job_root.join(&dwg.to_uppercase()).with_extension("PDF");
            let prelim = job_root
                .join("Preliminary")
                .join(&dwg.to_uppercase())
                .with_extension("PDF");

            if root.exists() {
                println!("{} from root(approved & released)", &dwg);
                args.push(root.to_str().unwrap().into());
            } else if prelim.exists() {
                println!("{} from preliminary", &dwg);
                args.push(prelim.to_str().unwrap().into());
            } else {
                println!("{} not found", &dwg)
            }
        }

        if args.len() > 0 {
            match get_pdf_handler() {
                Ok(pdf_handler) => {
                    Command::new(pdf_handler)
                        .args(&args)
                        .spawn()
                        .expect("Error opening PDF");
                }
                _ => {
                    println!("PDF file handler not found.");
                }
            }
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.open_files();

    Ok(())
}

fn parse_job(job: &str) -> String {
    String::from(job.to_uppercase())
}

fn parse_dwg(dwg: &str) -> Vec<String> {
    match dwg.split('-').collect::<Vec<&str>>()[..] {
        [_, _] => {
            lazy_static! {
                static ref PATTERN: Regex =
                    Regex::new(r"([a-zA-Z]+)([0-9]+)-[a-zA-Z]*([0-9]+)").unwrap();
            }

            let mut dwgs = vec![];
            match PATTERN.captures(dwg) {
                Some(caps) => {
                    let prefix = &caps[1];

                    // regex should ensure these two parse numerically and
                    // conventionally these numbers should never exceed u32::MAX
                    // therefore ->  should not panic!
                    let start: u32 = caps[2].parse().unwrap();
                    let end: u32 = caps[3].parse().unwrap();

                    for i in start..end + 1 {
                        dwgs.push(String::from(format!("{}{}", prefix, i)));
                    }

                    dwgs
                }
                None => vec![String::from(dwg)],
            }
        }
        _ => vec![String::from(dwg)],
    }
}

fn get_pdf_handler() -> std::io::Result<String> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let pdf: String = hkcr.open_subkey(".pdf")?.get_value("")?;
    let exec_path: PathBuf = [pdf.as_str(), "shell", "Open", "Command"].iter().collect();

    let exec: String = hkcr.open_subkey(exec_path)?.get_value("")?;

    let exe_pattern = Regex::new(r"C:[\w \\]+\.exe").unwrap();
    let mat = exe_pattern.find(&exec).unwrap();
    let exe = String::from(&exec[mat.start()..mat.end()]);

    Ok(exe)
}
