# i3-style

Make your [i3](http://i3wm.org) config a little more stylish.

**Warning:** i3-style is experimental. It may eat your config! Please back up your config file every time you use it until it is stable.

## About

Coming soon

## Installing

To install with [npm](https://npmjs.org/):

    npm install -g i3-style

The `i3-style` executable should now be in your PATH.

## Usage

Usage: `i3-style <theme> [options]`

Options:

  -c, --config `<file>`  The config file the theme should be applied to. Defaults to the default i3 config location.

  -o, --output `<file>`  Applies the theme, attempts to validate the result, and writes it to `<file>`. Prints to STDOUT if no output file is given.

`i3-style` applies the theme to your config file and prints the new config to stdout or writes it to the file specified by the `--output` argument. *The changes made to the config cannot be undone* so back up your existing config in case you don't like the new theme.

`<theme>` should be the path to an i3-style theme file or the name of one of the built-in styles. Theme files specify the colors i3 should use for window deocrations and the i3-bar. For example, to use a [solarized](http://ethanschoonover.com/solarized) colorscheme, create the file `solarized.yaml` with the contents:

```YAML
# solarized colorscheme by lasers (no cyan version)
---
colors:
  base03:    '#002b36'
  base02:    '#073642'
  base01:    '#586e75'
  base00:    '#657b83'
  base0:     '#839496'
  base1:     '#93a1a1'
  base2:     '#eee8d5'
  base3:     '#fdf6e3'
  yellow:    '#b58900'
  orange:    '#cb4b16'
  red:       '#dc322f'
  magenta:   '#d33682'
  violet:    '#6c71c4'
  blue:      '#268bd2'
  cyan:      '#2aa198'
  green:     '#859900'
  custom:    '#1c5766'
window_colors:
  focused:
    border:     'green'
    background: 'green'
    text:       'base3'
    indicator:  'blue'
  focused_inactive:
    border:     'custom'
    background: 'custom'
    text:       'base2'
    indicator:  'violet'
  unfocused:
    border:     'base02'
    background: 'base02'
    text:       'base1'
    indicator:  'base01'
  urgent:
    border:     'magenta'
    background: 'magenta'
    text:       'base3'
    indicator:  'red'
bar_colors:
  separator:          'blue'
  background:         'base03'
  statusline:         'base00'
  focused_workspace:
    border:           'green'
    background:       'green'
    text:             'base02'
  active_workspace:
    border:           'custom'
    background:       'custom'
    text:             'base2'
  inactive_workspace:
    border:           'base02'
    background:       'base02'
    text:             'base1'
  urgent_workspace:
    border:           'magenta'
    background:       'magenta'
    text:             'base3'
```

`i3-style` will apply this style by removing any existing style in your config and adding the new style to the client window decorations and to each bar config.

This style happens to be built into `i3-style` by default, so you can try it out by simply typing `i3-style solarized` and examining stdout. If you like what you see, back up your old config, create the styled config, and reload i3 to see what it looks like.

    $ cp ~/.i3/config ~/.i3/config.backup
    $ i3-style solarized -o ~/.i3/config
    $ i3-msg reload

And enjoy your new i3 theme. More themes are on the way!

## Contributing

Coming Soon

## License

Use only by the terms of the FreeBSD License (see LICENSE).

Copyright © 2013, Tony Crisci

All rights reserved.
