use std::io;
use std::process::ExitCode;

use rs_git_url2json::stdin2strings2jsons2stdout;

fn sub() -> Result<(), io::Error> {
    stdin2strings2jsons2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
