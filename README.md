# navigate

This repository implements a custom shell command for changing directories.
I implemented this program to learn rust (because I was put on a rust project at work) and because I have two major problems with bashs builtin `pushd`/`popd`/`dirs`:
* no option to suppress output on `pushd`/`popd`
* rotates stack when pushing to stack entry (with +/-<entry-number>)

## features

* `push` - save path to the stack and change to specified directory
* `pop` - pop one, or the specified amount of entries from the stack and move to the oldest one
* `stack` - display the stack
* `book` - move to/add/remove/display bookmarks

Every shell has its own stack, saved in the file `/tmp/navigate/<process-id>`.
`navigate` checks for and deletes orphaned stack files on execution.
This program does not run background tasks, all state is stored in temporary or configuration files.


## setup

`navigate` requires a setup
1) clone the repository
1) build crate
1) add path to executable to shell environment, or copy the executable to a directory in the path variable (e.g. `/usr/local/bin`)
1) source setup script `navigate_bash_setup` for convenience functions and bash completions


## configuration

The behaviour of `navigate` can be configured in the file `$XDG_CONFIG_HOME/navigate/navigate.toml`.

> *It has the toml extension, but might not implement the full toml specification.*

> `navigate` will check for the file `default.toml` in the configuration directory and create it if not found.
> It contains all settings with default values and a short explanation.
> After an update one can delete the file and call any subcommand of `navigate` to get an updated default configuration.

The lines without type and value are categories and need to be defined as toml table (`[table]`) in the configuration file.
Options are written as `value = key`.
Style settings accept styles and one color separated by commas.
Make sure to wrap the whole string in single or double quotes.
The following formats are supported:

* **styles**: `bold`, `dim`, `italic`, `underlined`, `blinking`, `reversed`, `invisible`, `strikethrough`
* **named color**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
* **numbered color**: `16`..`255`
* **rgb color**: `#rrggbb`

> *NOTE*: The styles and colors are applied as ansi escape sequences and your terminal may not support some of them.


# todos

- [ ] replace `std::io::Error` with `thiserror::Error`
- [x] option to pop several entries at a time and option to pop the entire stack
- [x] drop stack
- [x] config file
  - [x] implement procedural macro for config
    - [x] .. to parse config
    - [x] .. to write default config
    - [x] .. ignore comments in config file
  - [x] parse config file
  - [x] apply config -- partially more done than before :)
    - [x] `show-bookmarks-on-book`
  - [x] setting for separator string when displaying stack/bookmarks
  - [x] color option for punctuation (mostly '/')
  - [x] option to dedup stack entries
  - [x] option for behaviour when jumping to stack entry via `push =<n>`
- [x] bookmarks
  - [x] do not resolve links in bookmarks
  - [x] option to show invalid paths
    - [x] style option for invalid paths
  - [x] subcommand to remove invalid paths
    - [ ] print deleted bookmarks
- [x] push <number> to push path in stack
- [x] write documentation
- [x] change config file extension to `.toml`
- [x] add bash completions
- [x] option to show home directory as '~'
- [x] apply arguments or delete them..
