#!/usr/bin/env coffee

_ = require 'underscore'
fs = require 'fs'
sh = require 'shelljs'
program = require 'commander'
yaml = require 'js-yaml'
pathUtil = require 'path'
{mkConfig, mkTheme} = require './index'
VERSION = require('../package.json').version

sh.config.silent = yes

# convenience function to test existence of a file
fileExists = (path) -> path? and sh.test('-f', path)

# convenience function to exit with code 1 error. also prints usage.
exitWithError = (msg) ->
  program.outputHelp()
  process.stderr.write "Error: #{msg}\n"
  process.exit 1

program
  .version(VERSION)
  .usage('<theme> [options]')
  .option('-c, --config <file>', 'The config file the theme should be applied to. Defaults to the default i3 location')
  .option('-o, --output <file>', 'Apply the theme, attempt to validate the result, and write it to <file>')
  .option('-s, --save', 'Set the output file to the path of the input file')
  .option('-r, --reload', 'Apply the theme by reloading the config')
  .option('-l, --list-all', 'Print a list of all available themes')
  .option('-t, --to-theme [file]', 'Prints an i3-style theme based on the given config suitable for sharing with others')
  .parse(process.argv)

themesDir = pathUtil.resolve(__dirname, '../themes')
themesAvailable = sh.ls themesDir

# if --list-all was passed, print themes and exit
if program.listAll
  sh.echo '\n  Available themes:\n'
  themesList = []
  themesAvailable.forEach (themePath) ->
    theme = yaml.safeLoad sh.cat pathUtil.join(themesDir, themePath)
    paddedName = (themePath[i] or ' ' for i in [0..17]).join('')
    themesList.push "    #{paddedName} - #{theme.meta?.description or ''}"
  sh.echo themesList.join('\n') + '\n'
  process.exit 0

# get the contents of the config file specified on the command line, or look in
# the default locations for the config file
HOME = process.env.HOME
configPath = switch
  when program.config
    program.config
  when _.isString program.toTheme
    program.toTheme
  when HOME and fileExists "#{HOME}/.i3/config"
    "#{HOME}/.i3/config"
  when HOME and fileExists "#{HOME}/.config/i3/config"
    "#{HOME}/.config/i3/config"
  else
    exitWithError "Could not find a valid i3 config file"

# throw an error when a config file is not found
unless fileExists configPath
  exitWithError "Config file not found: #{configPath}"

if program.toTheme
  theme = mkTheme sh.cat configPath
  yamlTheme = """# vim: filetype=yaml
  ---
  #{yaml.safeDump theme}"""

  if program.output
    fs.writeFileSync(program.output, yamlTheme)
  else
    sh.echo yamlTheme

  process.exit 0

# print usage if no arguments
unless program.args.length
  program.outputHelp()
  process.exit 0

# throw an error if the theme file does not exist
unless fileExists(program.args[0]) or program.args[0] in themesAvailable
  exitWithError "Theme or file not found: #{program.args[0]}"

# try to parse the theme file they specified. it has to have the right file
# extension.
theme = switch
  when program.args[0].match /\.json$/
    JSON.parse sh.cat program.args[0]
  when program.args[0].match(/\.yaml$/) or program.args[0].match(/\.yml$/)
    yaml.safeLoad sh.cat program.args[0]
  when program.args[0] in themesAvailable
    yaml.safeLoad sh.cat pathUtil.join(themesDir, program.args[0])
  else
    yaml.safeLoad sh.cat program.args[0]

# do the heavy lifting
config = mkConfig theme, sh.cat(configPath)

# no output file specified. echo result to stdout and we are done.
outputPath = switch
  when program.output
    program.output
  when program.save
    configPath
  else
    null

unless outputPath
  sh.echo config
  sh.exec('i3-msg reload') if program.reload
  process.exit 0

# try to validate the generated config if we can
i3Path = sh.which 'i3'
tmpdir = sh.tempdir()

if i3Path and tmpdir
  # write the output config to the tmp directory
  tmpConfigPath = pathUtil.join tmpdir, 'i3-style-config'
  fs.writeFileSync tmpConfigPath, config
  if fileExists tmpConfigPath
    # try to validate it with the i3 -C flag
    validation = sh.exec("#{i3Path} -c #{tmpConfigPath} -C")
    sh.rm tmpConfigPath
    if validation.output.indexOf('ERROR:') > 0 or validation.code > 0
      exitWithError "Could not validate output configuration.\n\n#{validation.output}"

# make a backup (if not testing)
if tmpdir and not process.env.I3STYLETEST
  sh.mkdir pathUtil.join(tmpdir, 'i3-style')
  tmpPath = pathUtil.join(tmpdir, 'i3-style', "config.bak.#{Date.now()}")
  sh.echo "#{configPath} -> #{tmpPath}"
  sh.cp(configPath, tmpPath)

# finally write the file to the specified location
fs.writeFile outputPath, config, (err) ->
  exitWithError "Could not write to file: #{program.output}\n\n#{err}" if err
  sh.echo "Applied #{program.args[0]} theme to #{outputPath}"
  sh.exec('i3-msg reload') if program.reload
