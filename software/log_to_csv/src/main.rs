use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let re = Regex::new(r"VAR (?<var_name>\S*) (?<var_value>\S*)").unwrap();
    let mut csv_file = File::create("log_file.csv")?;
    let mut header: Option<&str> = None;
    // csv_file.write("UnixTime,p,v,h,filtered_v,filted_h\n".as_bytes())?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut to_write = String::new();
        for mat in re.find_iter(&line) {
            if let Some(captures) = re.captures(mat.as_str()) {
                if let (Some(m)) = captures.name("var_value") {
                    to_write = to_write + format!("{},", m.as_str()).as_ref();
                }
            }
        }
        if to_write.len() > 0 {
            let to_write = to_write.trim();
            let to_write = &to_write[..to_write.len() - 1];

            println!("to write {}", to_write);
            csv_file.write(format!("{}\n", to_write).as_bytes())?;
        }
    }
    Ok(())
}
