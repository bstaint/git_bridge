use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<_> = env::args().skip(1).collect();
    let output = Command::new("git")
        .args(&args)
        .output()?;

    let stdout = output.stdout;
    if let "rev-parse" = args.first().map(String::as_str).unwrap_or("") {
        let stdout = std::str::from_utf8(&stdout)?.trim();
        for line in stdout.split("\n") {
            let cygpath_output = Command::new("cygpath")
                .arg("-w")
                .arg(line).output()?;
            let stdout = std::str::from_utf8(&cygpath_output.stdout)?;
            println!("{}", stdout.trim());
        }
        io::stdout().flush()?;
    } else {
        io::stdout().write_all(&stdout)?;
    }
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}
