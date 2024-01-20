mod colors;
mod search;

use colors::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let args = parse_options(&args[1..]);

    let file_path = search::search();
    if file_path.is_none() {
        let path = search::ask_brew().unwrap_or(std::path::PathBuf::from("/opt/homebrew")); // not performance critical
        let path = path.display();

        println!("{RED}Error:{RESET} No such file or directory - {path}/Library/Taps/homebrew/homebrew-command-not-found/executables.txt");
        std::process::exit(1);
    }

    let file_path = file_path.unwrap();

    let mut results: Vec<(String, usize)> = vec![];

    let file = std::fs::read_to_string(file_path).unwrap();
    for line in file.lines() {
        if let Some(s) = parse_line(line, &args.formulae) {
            results.push((s.0.to_string(), s.1));
        }
    }

    let count: Vec<usize> = {
        let mut output = vec![0usize; args.formulae.len()];
        for formula in &results {
            output[formula.1] += 1;
        }

        output
    };

    if args.explain {
        for target in args.formulae.iter().enumerate() {
            if count[target.0] == 1 {
                println!(
                    "The program '{}' is currently not installed. You can install it by typing:",
                    target.1
                );
                for formula in &results {
                    if formula.1 == target.0 {
                        println!("  brew install {}", formula.0);
                    }
                }
            }

            if count[target.0] > 1 {
                println!(
                    "The program '{}' can be found in the following formulae:",
                    target.1
                );

                for formula in &results {
                    if formula.1 == target.0 {
                        println!("  * {}", formula.0);
                    }
                }

                println!("Try: brew install <selected formula>");
            }
        }
    } else {
        for formula in results {
            println!("{}", formula.0);
        }
    }
}

#[inline(always)]
fn parse_line<'a>(line: &'a str, targets: &'a [&str]) -> Option<(&'a str, usize)> {
    let mut f = line.find(':')? + 1;
    if f == line.len() {
        return None;
    }
    let mut b = 1
        + f
        + line[f + 1..]
            .find(char::is_whitespace)
            .unwrap_or_else(|| line[f + 1..].len());

    loop {
        for target in targets.iter().enumerate() {
            if line[f..b] == **target.1 {
                return Some((&line[0..line.find('(')?], target.0));
            }
        }

        f = 1 + f + line[f + 1..].find(char::is_whitespace)? + 1;
        b = 1
            + b
            + line[b + 1..]
                .find(char::is_whitespace)
                .unwrap_or_else(|| line[b + 1..].len());
    }
}

struct Arguments<'a> {
    explain: bool,
    debug: bool,
    quiet: bool,
    verbose: bool,
    formulae: Vec<&'a str>,
}

fn parse_options(args: &[String]) -> Arguments<'_> {
    let mut arguments = Arguments {
        explain: false,
        debug: false,
        quiet: false,
        verbose: false,
        formulae: vec![],
    };

    for arg in args {
        if *arg == "-h" || *arg == "--help" {
            print_help();
            std::process::exit(0);
        } else if *arg == "--explain" {
            arguments.explain = true;
        } else if *arg == "-q" || *arg == "--quiet" {
            arguments.quiet = true;
        } else if *arg == "-d" || *arg == "--debug" {
            arguments.debug = true;
        } else if *arg == "-v" || *arg == "--verbose" {
            arguments.verbose = true;
        } else if !arg.starts_with('-') {
            arguments.formulae.push(arg);
        }
    }

    arguments
}

fn print_help() {
    println!("{BOLD}Usage: brew_which_formula{RESET} [{BOLD}--explain{RESET}] {ULINE}command{RESET} [...]");
    println!();
    println!("Prints the formula(e) which provides the given command.");
    println!();
    println!("      --explain                    Output explanation of how to get 'cmd' by");
    println!("                                   installing one of the providing formulae.");
    println!("  -d, --debug                      Display any debugging information.");
    println!("  -q, --quiet                      Make some output more quiet.");
    println!("  -v, --verbose                    Make some output more verbose.");
    println!("  -h, --help                       Show this message.")
}
