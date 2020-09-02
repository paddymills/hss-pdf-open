use regex::Regex;
use std::path::PathBuf;
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

pub fn get_pdf_handler() -> std::io::Result<String> {
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
