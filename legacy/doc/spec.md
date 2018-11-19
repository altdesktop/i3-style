# The i3-config-builder Format Specification

<dl>
<dt>Authors</dt>
<dd>Tony Crisci (dubstepdish.com)</dd>

<dt>Revision</dt>
<dd>0.1.0</dd>

<dt>Date</dt>
<dd>10 April 2014</dd>

<dt>Copyright</dt>
<dd>Copyright © 2013 Tony Crisci. This work is licensed <a href="http://creativecommons.org/licenses/by-sa/3.0/us/">CC-BY-SA 3.0</a>
</dd>

<dt>Abstract</dt>
<dd>i3-config-builder is a format to create a context for templating <a href="http://i3wm.org">i3 window manager</a> config files. based on JavaScript Object Notation (JSON).</dd>
</dl>

## Introduction

i3-config-builder is a format for encoding context for i3 config templates.

A complete i3-config-builder data structure is a [JSON](http://www.json.org/) object, but may be expressed in formats that map to JSON such as [YAML](http://yaml.org/).

### Example

An i3-config-builder object for an i3 theme:

```YAML
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
    border:         'base1'
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
  background:       'base02'
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

## i3-config-builder Objects

An *i3-config-builder object* is always a single object. The i3-config-builder object may optionally have any of the members described below at its root. Each of the keys for the i3-config-builder object and all their members must only consist of word characters, digits, and the underscore.

### The Meta Object

The *meta object* may be present at the root of the i3-config-builder object with the key `meta`. If present, its value must be an object which may contain any of the following keys:

* `description`

If `description` is present, it must be a string less than 140 characters long and should briefly describe the purpose of the object.

### The Colors Object

The *colors object* may be present at the root of the i3-config-builder object with the key `colors`. If present, its value must be a single object with keys that each represent a *color alias*. The value of a color alias must be a single string value that expresses a valid *hex color code* in the same format as HTML hex color codes.

Other members of the i3-config-builder object may be specified to use color aliases in place of, or in addition to, hex color codes. Any other member with such a specificiation must satisfy:

1. It must have exactly one member which must be specified to be exclusively either a string or an object with string values but not both.
2. The string value or any member value of object each must be either a color alias such that there is a key in the colors object equal to that value or a valid hex color code.

### The Window Colors Object

The *windows colors object* may be present at the root of the i3-config-builder object with the key `window_colors`. If present, its value must be an object which may contain any of the following keys:

* `focused`
* `focused_inactive`
* `unfocused`
* `urgent`

For each of these keys that are present, its value must be an object which may contain any of the following keys:

* `border`
* `background`
* `text`
* `indicator`

For each of these keys that are present, its value must be a single string which is either a color alias or a hex color code.

### The Bar Colors Object

The bar colors object may be present at the root of the i3-config-builder object with the key `bar_colors`. If present, its value must be an object which may contain any of the following keys:

* `separator`
* `background`
* `statusline`
* `focused_workspace`
* `active_workspace`
* `inactive_workspace`
* `urgent_workspace`

If the any of the keys `separator`, `background`, or `statusline`, are present, its value must be a single string which is either a color alias or a hex color code.

If any of the keys `focused_workspace`, `active_workspace`, `inactive_workspace`, or `urgent_workspace` are present, its value must be a single object which may contain any of the following keys:

* `border`
* `background`
* `text`

If any of these keys are present, its value must be a single string which is either a color alias or a hex color code.
