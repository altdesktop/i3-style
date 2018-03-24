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

Just call `i3-style` with the name of the theme you want to try and where you want to write the config file to. i3-style will look for your config in the default place and apply the theme.

    i3-style solarized -o ~/.config/i3/config --reload

Check the `themes` directory for the list of built-in themes.

If you want to modify a theme, copy it from `themes` and give it a `.yaml` extension. The object format is [well-documented](https://github.com/acrisci/i3-style/blob/master/doc/spec.md) and includes support for color aliases. Then back up your config and call i3-style.

    i3-style ~/.config/i3/solarized.yaml -o ~/.config/i3/config

Just keep doing that until you get it perfect (which might be never).

## Send us themes!

Do you have a cool colorscheme in your config file that you want to share with other people? i3-style can automatically convert it to a theme file:

    i3-style --to-theme ~/.config/i3/config -o my-theme.yaml

If you have a new theme, or made an improvement to an existing theme, please make a pull request adding your theme to the `themes` directory!

## License

This work is available under a FreeBSD License (see LICENSE).

Copyright © 2013, Tony Crisci

All rights reserved.
