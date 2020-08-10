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
            let root = job_root.join(&dwg).with_extension("PDF");
            let prelim = job_root
                .join("Preliminary")
                .join(&dwg)
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

    // TODO: launch ViewShopDrawings if no args
    // PathBuf::from(r"\\HSSIENG\Resources\HS\PROG\DCS\ViewShopDrawingsNET.exe")
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

            match PATTERN.captures(dwg) {
                Some(caps) => {
                    let mut dwgs = vec![];
                    let prefix = &caps[1];
                    let (a, b) = (&caps[2], &caps[3]);

                    // regex should ensure these two parse numerically and
                    // conventionally these numbers should never exceed u32::MAX
                    // therefore ->  should not panic!
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
    }
}

fn get_pdf_handler() -> std::io::Result<String> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // get pdf handler
    let pdf: String = hkcr.open_subkey(".pdf")?.get_value("")?;

    // get location of pdf handler
    let exec_path: PathBuf = [pdf.as_str(), "shell", "Open", "Command"].iter().collect();
    let exec: String = hkcr.open_subkey(exec_path)?.get_value("")?;

    // parse out exe only (may have shell argument placeholders in value)
    let exe_pattern = Regex::new(r"C:[\w \\]+\.exe").unwrap();
    let mat = exe_pattern.find(&exec).unwrap();
    let exe = String::from(&exec[mat.start()..mat.end()]);

    Ok(exe)
}
