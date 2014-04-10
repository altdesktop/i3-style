# i3-style

Make your [i3](http://i3wm.org) config a little more stylish.

[![NPM](https://nodei.co/npm/i3-style.png?compact=true)](https://nodei.co/npm/i3-style/)
[![NPM](https://nodei.co/npm-dl/i3-style.png?months=6)](https://nodei.co/npm/i3-style/)

## About

i3-style applies a theme to your i3 config file to change the colorscheme of the window decorations and the different parts of i3bar. It's designed especially for people who make frequent changes to their colorscheme to get things just right.

* Easy to try out new themes right after you install.
* Themes are easy to read, modify, and share.
* Modifies your theme in place - extra template files are not needed.

For an overview of the capabilities of i3-style, see my [blog post](http://dubstepdish.com/blog/2013/11/06/introducing-i3-style/).

## Installing

To install with [npm](https://npmjs.org/):

    npm install -g i3-style

The `i3-style` executable should now be in your PATH.

## Usage

First of all, it's always a good idea to make a backup of your config before you try a new theme so you can get things back to how they were. Then just call `i3-style` with the name of the theme you want to try and where you want to write the config file to. i3-style will look for your config in the default place and apply the theme. Then just restart i3 and enjoy the new theme.

    cp ~/.i3/config ~/.i3/config.backup
    i3-style solarized -o ~/.i3/config
    i3-msg restart

Check the `themes` directory for the list of built-in themes.

If you want to modify a theme, copy it from `themes` and give it a `.yaml` extension. The object format is [well-documented](https://github.com/acrisci/i3-style/blob/master/doc/spec.md) and includes support for color aliases. Then back up your config and call i3-style.

    i3-style ~/.i3/solarized.yaml -o ~/.i3/config

Just keep doing that until you get it perfect (which might be never).

## Example theme

```yaml
# solarized colorscheme by lasers (no cyan version)
---
colors:
  base03:           '#002b36'
  base02:           '#073642'
  base01:           '#586e75'
  base00:           '#657b83'
  base0:            '#839496'
  base1:            '#93a1a1'
  base2:            '#eee8d5'
  base3:            '#fdf6e3'
  yellow:           '#b58900'
  orange:           '#cb4b16'
  red:              '#dc322f'
  magenta:          '#d33682'
  violet:           '#6c71c4'
  blue:             '#268bd2'
  cyan:             '#2aa198'
  green:            '#859900'
  custom:           '#1c5766'
window_colors:
  focused:
    border:         'green'
    background:     'green'
    text:           'base3'
    indicator:      'green'
  focused_inactive:
    border:         'base02'
    background:     'base02'
    text:           'base2'
    indicator:      'violet'
  unfocused:
    border:         'base02'
    background:     'base02'
    text:           'base1'
    indicator:      'base01'
  urgent:
    border:         'magenta'
    background:     'magenta'
    text:           'base3'
    indicator:      'red'
bar_colors:
  separator:        'red'
  background:       'base03'
  statusline:       'blue'
  focused_workspace:
    border:         'base3'
    background:     'green'
    text:           'base3'
  active_workspace:
    border:         'base3'
    background:     'violet'
    text:           'base3'
  inactive_workspace:
    border:         'base01'
    background:     'base1'
    text:           'base03'
  urgent_workspace:
    border:         'magenta'
    background:     'magenta'
    text:           'base3'
```

## Send us themes!

If you've made a new theme, or made an improvement to an existing theme, please make a pull request adding your theme to the `themes` directory!

## License

This work is available under a FreeBSD License (see LICENSE).

Copyright © 2013, Tony Crisci

All rights reserved.
