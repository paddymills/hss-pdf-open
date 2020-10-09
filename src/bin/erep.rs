use exitfailure::ExitFailure;
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

use hsspdfopen::get_pdf_handler;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, StructOpt)]
#[structopt(name = "View Shop Drawings", about = "Shop Drawing file launcher")]
struct Cli {
  #[structopt(parse(from_str = parse_prog))]
  progs: Vec<Vec<String>>,
}

impl Cli {
  fn open_files(self) {
    const PROG_LEN: usize = 5; // must be usize for len comparison and trucate
    let mut last_prog = String::from("00000");

    let erep_root = PathBuf::from(r"\\hssfileserv1\hssiserv1\Shops\eReports91");

    let mut args: Vec<String> = Vec::new();
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
        println!("Opening: {}", &prog);
        args.push(root.to_str().unwrap().into());
      } else {
        println!("{} not found", &prog)
      }

      last_prog = prog;
      if last_prog.len() > PROG_LEN {
        last_prog.truncate(PROG_LEN);
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

fn parse_prog(prog: &str) -> Vec<String> {
  match prog.split('-').collect::<Vec<&str>>()[..] {
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
            progs.push(i.to_string());
          }

          progs
        }
        None => vec![String::from(prog)],
      }
    }
    _ => vec![String::from(prog)],
  }
}
