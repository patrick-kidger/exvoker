<h1 align="center">exvoker</h1>
<p align="center"><em>Extract a regex, invoke a command on it.</em></p>

The canonical use (that I wrote this for) is to extract URLs from stdout on the command line, and then automatically copy them to clipboard or open them in a web browser.

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

**Commands**

- `Esc`/`Control-C`: _quit (and do nothing)_
- `Up`/`Shift-Tab`: _move selection up_
- `Down`/`Tab`: _move selection down_
- `Home`/`End`/`Left`/`Right`/letters/numbers/punctuation/etc.: _edit fuzzy selection_
- `Space`: _select item_
- `Enter`: _select item and edit it (using the same interface as fuzzy selection). Then press either `Space` or `Enter` again to select the edited version._

## tmux integration

I have `bind e run tmux-exvoker` in my tmux configuration file, and the file `tmux-evoker` on my `$PATH`, with contents:
```fish
#!/usr/bin/env fish
set file (mktemp -p /dev/shm)
tmux capture-pane -pJ > $file
tmux new-window -n "<exvoker>" "cat $file | exvoker"
```
(you may need to adapt this if you use other shells, e.g. bash or zsh)

This will call exvoker on the currently-visible contents of your pane, when you press `<prefix> e`.

## Invoke a web browser

If you want to open the selected link in a web browser, then this can usually be accomplished by setting:
```toml
invoke = "xdg-open"
```

## Inspiration

The basic functionality of copying URLs from stdout to clipboard was inspired by the venerable `urlview`. This is intended as a personal replacement for that. However I didn't really like it enough (or that of its other derivatives), mostly because it involved too many keypresses. For example extracting a URL via `urlview` involves three keypresses, to select item -> edit item -> close menu. In contrast exvoker only requires a single keypress to select an item.

The command line interface is adapted from [dialoguer](https://github.com/console-rs/dialoguer), which honestly does most of the heavy lifting here.
