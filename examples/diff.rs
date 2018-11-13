extern crate lcs;

use lcs::{Diff, Ordering, Lcs};

use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader}
};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let (source, dest) = match (args.next(), args.next()) {
        (Some(source), Some(dest)) => (source, dest),
        _ => {
            eprintln!("diff <source> <dest>");
            return Ok(());
        }
    };

    let source: io::Result<Vec<String>> = BufReader::new(File::open(source)?).lines().collect();
    let dest:   io::Result<Vec<String>> = BufReader::new(File::open(dest)?).lines().collect();

    let source = source?;
    let dest = dest?;

    let lcs = Lcs::new(&source, &dest);

    for diff in lcs.backtrack(Ordering::DeleteFirst) {
        match diff {
            Diff::Common(line) => println!("  {}", line),
            Diff::Delete(line) => println!("- {}", line),
            Diff::Insert(line) => println!("+ {}", line),
        }
    }

    Ok(())
}
