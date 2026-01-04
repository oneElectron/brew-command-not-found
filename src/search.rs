use std::{env, path::PathBuf};

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
    let root = ask_default_root().or_else(|| ask_brew())?;
    Some(root.join("api").join("internal").join("executables.txt"))
}

pub(crate) fn ask_brew() -> Option<PathBuf> {
    let output = std::process::Command::new("brew")
        .arg("--cache")
        .output()
        .ok()?;

    let repo_path = std::string::String::from_utf8(output.stdout).ok()?;
    let repo_path = PathBuf::from(repo_path.trim().to_string());

    if repo_path.exists() {
        return Some(repo_path);
    }

    None
}

pub(crate) fn ask_default_root() -> Option<PathBuf> {
    let path = env::home_dir()?.join(".cache").join("Homebrew");

    if path.exists() {
        return Some(path);
    }

    None
}
