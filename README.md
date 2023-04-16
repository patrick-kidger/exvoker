<h1 align="center">exvoker</h1>
<p align="center"><em>Extract a regex, invoke a command on it.</em></p>

The canonical use (that I wrote this for) is to extract URLs from stdout on the command line, and then automatically copy them to clipboard or open them in a web browser.

<div align="center"><img width=600 src="./imgs/exvoker.gif"></div>

## Installation

```
cargo install --locked --git https://github.com/patrick-kidger/exvoker
```

You'll also need to create a `~/.config/exvoker/exvoker.toml` file specifying what regexes to extract, and what command to run on them. For example, here's mine:
```toml
extract = '''(http|https)://\S+'''
invoke = 'tmux set-buffer -w'
```
This extracts most URLs, and copies the selected item to my tmux clipboard.  
(This URL-matching is deliberately simplistic, and you can try other more complicated regexes if you want to.)

## Usage

exvoker reads from stdin and parses out items that match the regex. Using the URL regex above, then this:
```
echo "https://github.com foo bar http://google.com qux https://twitter.com" | exvoker
```

produces:
<pre>
  ▓
❯ https://github.com
  http://google.com
  https://twitter.com
</pre>

This menu can be navigated (keys listed below), and also has a fuzzy search prompt on the first line:
<pre>
  go▓
❯ http://<b>go</b>ogle.com
  https://<b>g</b>ithub.c<b>o</b>m
</pre>

Once you're happy with your choice, then press `Space` to select the item, which will run your `invoke` command on it. (Passed as `$invoke $selection`.)  
Alternatively, press `Enter` to edit the item before invoking a command on it.

#### Commands

- `Esc`/`Control-C`: _quit (and do nothing)_
- `Up`/`Shift-Tab`: _move selection up_
- `Down`/`Tab`: _move selection down_
- `Home`/`End`/`Left`/`Right`/letters/numbers/punctuation/etc.: _edit fuzzy selection_
- `Space`: _select item_
- `Enter`: _select item and edit it (using the same interface as fuzzy selection). Then press either `Space` or `Enter` again to select the edited version._

## tmux integration

I have `bind e run tmux-exvoker` in my tmux configuration file, and the executable file `tmux-evoker` on my `$PATH` (make sure it's executable: `chmod +x tmux-exvoker`), with contents:
```fish
#!/usr/bin/env fish
set file (mktemp -p /dev/shm)
tmux capture-pane -pJ > $file
tmux new-window -n "<exvoker>" "cat $file | exvoker"
```
(you may need to adapt this if you use other shells, e.g. bash or zsh)

This will call exvoker on the currently-visible contents of your pane, when you press `<prefix> e`.

## Examples

#### Open a web browser

To open the selected link in a web browser, include this in `exvoker.toml`:
```toml
invoke = 'xdg-open'
```

#### Copy to the system clipboard

Depends exactly what is meant by "system" clipboard, but on many Linux distros that frequently means using the X clipboard:
```toml
invoke = 'xsel -ib'
```

#### Copy to both the tmux clipboard and the system clipboard

In `exvoker.toml`:
```toml
invoke = 'tmux set-buffer -w'
```

#### Automatically rewrite some matches

In this example, we additionally match strings of the form `Index <number>`, and rewrite them to `https://example.org/index/<number>`, and then invoke our final command.

In `exvoker.toml`:
```toml
extract = '''((http|https)://\S+|Index [0-9]+)'''
invoke = 'exvoker-invoke'
```

And in an file `exvoker-invoke` somewhere on your `$PATH` (make sure it's executable: `chmod +x exvoker-invoke`):
```fish
#!/usr/bin/env fish
if test (count $argv) -ne 1
    echo "Can only invoke with a single argment"
    exit 1
end
set --function arg $argv[1]
if test (string match --regex '^Index [0-9]+' "$arg")
    set --function arg (string replace 'Index ' 'https://example.org/index/' "$arg")
end
tmux set-buffer -w $arg  # Or whatever else your final command is
```
The above uses the fish shell, but you could construct something equivalent if you're a bash/zsh/etc. user.

## Inspiration

The basic functionality of copying URLs from stdout to clipboard was inspired by the venerable `urlview`. This is intended as a personal replacement for that. I didn't really like `urlview` enough (or any of its derivatives), mostly because they involved too many keypresses. For example extracting a URL via `urlview` involves three keypresses, to select item -> edit item -> close menu. In contrast exvoker only requires a single keypress to select an item.

The command line interface is adapted from [dialoguer](https://github.com/console-rs/dialoguer), which honestly does most of the heavy lifting here.
