# Homebrew Command Not Found


## Usage
This is kind of a hack currently and only serves as a proof of concept.

You must first run `brew which-formula` at least once:
```
brew which-formula bash
```

Use cargo to install:
```bash
cargo install --git https://github.com/oneElectron/brew-command-not-found
```
or:
```bash
git clone https://github.com/oneElectron/brew-command-not-found
cd brew-command-not-found
cargo install --path .
```

__WARNING: This is a hack to get the proof of concept working, follow these steps at your own risk.__

Follow the instructions in: ```brew command-not-found-init```.
Part of the shell commands will contain a file with a name like `handler.sh`.
In that file, find and replace ```brew which-formula --explain $cmd 2>/dev/null``` with:
```bash
brew_which_formula --explain $cmd 2> /dev/null
```

Restart your shell, and if everything works out, any incorrect command you type will take milliseconds to look up.
