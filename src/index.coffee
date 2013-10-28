_ = require 'underscore'

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
  bar\s*{
    ([^}]*)
  }
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
mkWindowColors = (colors, windowColors) ->
  result = []
  _.each windowColors, (parts, state) ->
    result.push templates.windowColors
      state: state
      border: colors[parts.border]
      background: colors[parts.background]
      text: colors[parts.text]
      indicator: colors[parts.indicator]
  return result.join '\n'

# takes color aliases and bar color directive object and returns i3 config bar
# color directive string
mkBarColors = (colors, bar_colors) ->
  result = []
  _.each bar_colors, (parts, state) ->
    if _.isString parts
      result.push "#{state} #{colors[parts]}"
    else
      result.push templates.barColors
        state: state
        border: colors[parts.border]
        background: colors[parts.background]
        text: colors[parts.text]
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
  config += "\n#{mkWindowColors theme.colors, theme.window_colors}"
  return config

module.exports =
  mkConfig: mkConfig
