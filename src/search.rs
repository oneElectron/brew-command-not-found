use std::path::PathBuf;

#[cfg(target_arch = "x86_64")]
static DEFAULT_PATH: &str =
    "/usr/homebrew/Library/Taps/homebrew/homebrew-command-not-found/executables.txt";

#[cfg(target_arch = "aarch64")]
static DEFAULT_PATH: &str =
    "/opt/homebrew/Library/Taps/homebrew/homebrew-command-not-found/executables.txt";

/*
 *
 * This is an experiment, this is only really useful if it takes a long time,
 * but it does, hence why it is commented out
pub(crate) fn search_parallel() -> Option<PathBuf> {
    let start_time = std::time::Instant::now();
    let default_path = std::thread::spawn(ask_default_path);

    let brew_path = std::thread::spawn(ask_brew);

    if let Some(p) = default_path.join().unwrap() {
        let end_time = std::time::Instant::now();
        println!(
            "Time taken default: {} micros",
            (end_time - start_time).as_micros()
        );
        return Some(p);
    }

    if let Some(p) = brew_path.join().unwrap() {
        let end_time = std::time::Instant::now();
        println!(
            "Time taken brew: {} micros",
            (end_time - start_time).as_micros()
        );
        return Some(p);
    }

    None
}
*/

pub(crate) fn search() -> Option<PathBuf> {
    let default_path = ask_default_path();
    if default_path.is_some() {
        return default_path;
    }

    let brew_path = ask_brew();
    if brew_path.is_some() {
        return brew_path;
    }

    None
}

pub(crate) fn ask_brew() -> Option<PathBuf> {
    let output = std::process::Command::new("brew")
        .arg("--repository")
        .output();
    if output.is_err() {
        return None;
    }

    let repo_path = std::string::String::from_utf8(output.unwrap().stdout);
    if repo_path.is_err() {
        return None;
    }

    let mut repo_path = repo_path.unwrap().trim().to_string();
    repo_path.push_str("/Library/Taps/homebrew/homebrew-command-not-found/executables.txt");

    let path = PathBuf::from(repo_path);

    if path.exists() {
        return Some(path);
    }

    None
}

pub(crate) fn ask_default_path() -> Option<PathBuf> {
    let path = PathBuf::from(DEFAULT_PATH);

    if path.exists() {
        return Some(path);
    }

    None
}
