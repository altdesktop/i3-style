chai = require 'chai'
{mkConfig} = require '../src'
theme = require './theme.json'
sh = require 'shelljs'
chai.should()

sh.config.silent = yes
process.env.I3STYLETEST = "yes"

commentedBar = """
#bar {}
#bar {
#stuff
#}
"""

normalBar = """
bar {
  verbose true
  # @bar_comment@
  colors {
  }
}
"""

testForColors = (config) ->
  /client\.focused #859900 #859900 #fdf6e3 #859900/.test(config).should.be.true
  /client\.focused_inactive #073642 #073642 #eee8d5 #6c71c4/.test(config).should.be.true
  /client\.unfocused #073642 #073642 #93a1a1 #586e75/.test(config).should.be.true
  /client\.urgent #d33682 #d33682 #fdf6e3 #dc322f/.test(config).should.be.true

testForBarColors = (config) ->
  /separator #dc322f/.test(config).should.be.true
  /background #002b36/.test(config).should.be.true
  /statusline #268bd2/.test(config).should.be.true
  /active_workspace #fdf6e3 #6c71c4 #fdf6e3/.test(config).should.be.true
  /inactive_workspace #586e75 #93a1a1 #002b36/.test(config).should.be.true
  /focused_workspace #fdf6e3 #859900 #fdf6e3/.test(config).should.be.true
  /urgent_workspace #d33682 #d33682 #fdf6e3/.test(config).should.be.true

describe 'Bar block', ->
  result = mkConfig(theme, normalBar)
  it 'should add a colors block', ->
    /colors {/.test(result).should.be.true
  it 'should preserve bar comments and directives', ->
    /# @bar_comment@/.test(result).should.be.true
    /verbose true/.test(result).should.be.true
  it 'should add colors to the colors block', ->
    testForBarColors result
  it 'should add colors to the end', ->
    testForColors result

describe 'Config with comments', ->
  result = mkConfig(theme, commentedBar)
  it 'should ignore bar blocks within comments', ->
    /colors {/.test(result).should.be.false

describe 'Default config', ->
  it 'should apply the theme with no errors', ->
    # you need to have `i3` in your PATH for this to work
    {code} = sh.exec('src/cli.coffee solarized -c test/default-config -o /dev/null')
    code.should.be.equal 0
  it 'should apply the theme to the default config', ->
    {output, code} = sh.exec('src/cli.coffee -c test/default-config test/theme.json')
    code.should.be.equal 0
    testForColors output
    testForBarColors output
