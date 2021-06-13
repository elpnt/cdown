# cdown

`cdown` is a cross-platform TUI timer app purely written in Rust.

<p align="center">
  <img src="https://user-images.githubusercontent.com/39664774/121809504-fb1c5c80-cc97-11eb-821a-fdfe4b98203f.gif" alt="demo" width="600">
</p>

# Usage

```sh
Hotkeys:
    p       Pause/Resume
    q/ESC   Quit

USAGE:
    cdown [FLAGS] [OPTIONS] [time]

FLAGS:
    -b                   Display a box border around the timer
    -h, --help           Prints help information
    -l, --list-colors    Prints list of available colors
    -V, --version        Prints version information

OPTIONS:
    -c <color>        Set the foreground color 

ARGS:
    <time>     [default: 3min]
```
