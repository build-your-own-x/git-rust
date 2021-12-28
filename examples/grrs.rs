#![allow(unused)]

use std::path::PathBuf;

use anyhow::Context;
use structopt::StructOpt;

// reference: https://rust-cli.github.io/book/index.html
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// By default, it'll only report errors. Passing -v one time also prints warnings, -vv enables info logging, -vvv debug, and -vvvv trace.
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::from_args();
    println!("{:?}", args);
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    crate::find_matches(&content, &args.pattern, &std::io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let mut result = Vec::new();
        crate::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
