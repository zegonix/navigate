# navigate

This repository implements a custom shell command for changing directories.
I implemented this program to learn rust (because I was put on a rust project at work) and because I have two major problems with bashs builtin `pushd`/`popd`/`dirs`:
* no option to suppress output on `pushd`/`popd`
* rotates stack when pushing to stack entry (with +/-<entry-number>)

## features

* `push` - save path to the stack and change to specified directory
* `pop` - pop one, or the specified amount of entries from the stack and move to the oldest one
* `stack` - display the stack
* `book` - move to, or add, remove and display bookmarks

Every shell has its own stack, save in the file `/tmp/navigate/<process-id>`.
`navigate` checks for and deletes orphaned stack files on execution.


## setup

`navigate` requires a setup
1) clone the repository
1) build crate
1) add path to executable to shell environment
1) source setup script `navigate_bash_setup`


## configuration

The behaviour of `navigate` can be configured in the file `$XDG_CONFIG_HOME/navigate/navigate.toml`.
*It has the toml extension, but might not implement the full toml specification.*

At the time of writing, the following options are recognised:
* `general`
  * `show_stack_on_push`: bool = false
  * `show_stack_on_pop`: bool = false
  * `show_books_on_bookmark`: bool = false
* `format`
  * `align_separators`: bool = true
  * `stack_separator`: string = " - "
  * `stack_home_as_tilde`: bool = false
  * `bookmarks_separator`: string = " - "
  * `book_home_as_tilde`: bool = false
* `styles`
  * `warning_style`: string = "yellow, italic"
  * `error_style`: string = "red, bold"
  * `stack_number_style`: string = "default"
  * `stack_separator_style`: string = "cyan"
  * `stack_path_style`: string = "default"
  * `stack_punct_style`: string = "magenta"
  * `bookmarks_name_style`: string = "default"
  * `bookmarks_separator_style`: string = "cyan"
  * `bookmarks_path_style`: string = "default"
  * `bookmarks_punct_style`: string = "magenta"

The lines without type and value are categories and need to be defined as toml table (`[table]`) in the configuration file.
Options are written as `value = key`.
Style settings accept styles and one color in the following formats:
* **styles**: `bold`, `dim`, `italic`, `underlined`, `blinking`, `reversed`, `invisible`, `strikethrough`
* **named color**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
* **numbered color**: `16`..`255`
* **rgb color**: `#rrggbb`

***NOTE**: The styles and colors are applied as ansi escape sequences and most terminals do not support all options.*

See [navigate.toml](./navigate.toml) for an example configuration file.

