# cdown

`cdown` is a timer TUI app purely written in Rust.

![demo](https://github.com/elpnt/cdown/blob/screenshot/demo.gif)

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