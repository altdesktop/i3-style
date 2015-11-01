_ = require 'underscore'
namer = require 'color-namer'

# change to mustache style templates
_.templateSettings =  interpolate :/\{\{(.+?)\}\}/g

# templates to build lines in the config
templates =
  windowColors: _.template "client.{{state}} {{border}} {{background}} {{text}} {{indicator}}"
  barColors: _.template "{{state}} {{border}} {{background}} {{text}}"

# matches the bar directive and captures the inside
# IMPORTANT: only works if the bar doesn't have colors directive! strip first!
# TODO: fix this so it will ignore inner braces
matchBar = ///
  bar\s*{\s*\n
    ([^}]*)
  \n}
///g

# matches a colors directive, such as may be inside a bar directive
matchBarColors = ///
  \s*colors\s*{
    [^}]*
  }
///g

# matches a client color directive
matchWindowColors = ///
  \s*client\.
  (?:focused | focused_inactive | unfocused | urgent)
  \s+
  .*
///g

# strips the config of any (bar) color directives and client color directives
stripConfig = (config) ->
  config.replace(matchWindowColors, '').replace(matchBarColors, '')

# takes color aliases and window_color directive object and returns i3 config
# client color directives string
mkWindowColors = (colors = {}, windowColors) ->
  result = []
  _.each windowColors, (parts, state) ->
    result.push templates.windowColors
      state: state
      border: colors[parts.border] or parts.border
      background: colors[parts.background] or parts.background
      text: colors[parts.text] or parts.text
      indicator: colors[parts.indicator] or parts.indicator
  return result.join '\n'

# takes color aliases and bar color directive object and returns i3 config bar
# color directive string
mkBarColors = (colors = {}, bar_colors) ->
  result = []
  _.each bar_colors, (parts, state) ->
    if _.isString parts
      result.push "#{state} #{colors[parts] or parts}"
    else
      result.push templates.barColors
        state: state
        border: colors[parts.border] or parts.border
        background: colors[parts.background] or parts.background
        text: colors[parts.text] or parts.text
  return result.join "\n    "


# takes theme object and i3 config string and returns a new i3 config string
# with the theme applied
mkConfig = (theme, config) ->
  config = stripConfig config
  config = config.replace matchBar, (match, inside) ->
    return """
    bar {
      #{inside.trim()}
      colors {
        #{mkBarColors theme.colors, theme.bar_colors}
      }
    }
    """
  config += "\n#{mkWindowColors theme.colors, theme.window_colors}\n"
  return config

# takes a config and returns an i3-style theme
mkTheme = (config) ->
  theme =
    meta:
      description: 'AUTOMATICALLY GENERATED THEME'

  # add a color name to the theme for the given hex color and return the name
  # of the color
  addColor = (hex) ->
    if not hex
      return null

    unless theme.colors?
      theme.colors = {}

    colorName = namer(hex).html[0].name

    if theme.colors[colorName]? and theme.colors[colorName] isnt hex
      i = 0
      while true
        i += 1
        c = "#{colorName}#{i}"
        if not theme.colors[c] or theme.colors[c] == hex
          colorName = c
          break

    theme.colors[colorName] = hex
    return colorName

  stateBarInside = off
  stateColorsInside = off

  for line in config.split('\n')
    # trim the line
    line = line.replace(/^\s+|\s+$/g, '')
    # remove duplicate spaces between words
    line = line.replace(/\s+/g, ' ')

    # beginning of a bar block
    if line.indexOf('bar {') is 0
      stateBarInside = on
      continue

    # beginning of a bar color block
    if line.indexOf('colors {') is 0 and stateBarInside
      stateColorsInside = on
      continue

    if line.indexOf('}') is 0
      if stateColorsInside
        # end of a bar color block
        stateColorsInside = off
        continue
      if stateBarInside
        # end of a bar block
        stateBarInside = off
        continue

    # check for window color config directive
    if line.indexOf('client.') == 0
      color = line.split(' ')
      color[0] = color[0].substring('client.'.length)
      if color[0] in ['focused', 'focused_inactive', 'unfocused', 'urgent']
        border = addColor color[1]
        background = addColor color[2]
        text = addColor color[3]
        indicator = addColor color[4]

        unless theme.window_colors?
          theme.window_colors = {}

        theme.window_colors[color[0]] = {}
        if border? then theme.window_colors[color[0]].border = border
        if background? then theme.window_colors[color[0]].background = background
        if text? then theme.window_colors[color[0]].text = text
        if indicator? then theme.window_colors[color[0]].indicator = indicator

      continue

    # check for bar color config directive
    if stateColorsInside
      barColor = line.split(' ')
      if barColor[0] in ['separator', 'background', 'statusline']
        unless theme.bar_colors?
          theme.bar_colors = {}

        theme.bar_colors[barColor[0]] = addColor barColor[1]
      else if barColor[0] in ['focused_workspace', 'active_workspace',
                              'inactive_workspace', 'urgent_workspace']
        border = addColor barColor[1]
        background = addColor barColor[2]
        text = addColor barColor[3]

        unless theme.bar_colors?
          theme.bar_colors = {}

        theme.bar_colors[barColor[0]] = {}
        if border? then theme.bar_colors[barColor[0]].border = border
        if background? then theme.bar_colors[barColor[0]].background = background
        if text? then theme.bar_colors[barColor[0]].text = text

      continue

  return theme

module.exports =
  mkConfig: mkConfig
  mkTheme: mkTheme
