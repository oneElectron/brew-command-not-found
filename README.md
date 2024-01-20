# Homebrew Command Not Found


## Usage
This is kind of a hack currently and only serves as a proof of concept.

You must have the official homebrew-command-not-found tapped:
```
brew tap homebrew/command-not-found
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

Find the shell loader in either: ```/opt/homebrew/Library/Taps/homebrew/homebrew-command-not-found/handler.sh``` or: ```/usr/homebrew/Library/Taps/homebrew/homebrew-command-not-found/handler.sh```.
If you are using fish replace ```handler.sh``` with ```handler.fish```

Then find and replace ```brew which-formula --explain $cmd 2>/dev/null``` with:
```bash
brew_which_formula --explain $cmd 2> /dev/null
```

restart your shell, and if everything works out, any incorrect command you type will take milliseconds to look up.