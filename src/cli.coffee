#!/usr/bin/env coffee

fs = require 'fs'
sh = require 'shelljs'
program = require 'commander'
yaml = require 'js-yaml'
pathUtil = require 'path'
{mkConfig} = require './index'

sh.config.silent = yes

program
  .version('0.0.3')
  .usage('<theme> [options]')
  .option('-c, --config <file>', 'The config file the theme should be applied to. Defaults to the default i3 location.')
  .option('-o, --output <file>', 'Applies the theme, attempts to validate the result, and writes it to <file>. Prints to STDOUT if no output file is given.')
  .parse(process.argv)

# convenience function to test existence of a file
fileExists = (path) -> path? and sh.test('-f', path)

# convenience function to exit with code 1 error. also prints usage.
exitWithError = (msg) ->
  program.outputHelp()
  process.stderr.write "Error: #{msg}"
  process.exit 1

# throw an error if the theme file does not exist
themesDir = pathUtil.resolve(__dirname, '../themes')
themesAvailable = sh.ls themesDir

unless fileExists(program.args[0]) or program.args[0] in themesAvailable
  exitWithError "Theme or file not found: #{program.args[0]}"

# try to parse the theme file they specified. it has to have the right file
# extension.
theme = switch
  when program.args[0].match /\.json$/
    JSON.parse sh.cat program.args[0]
  when program.args[0].match /\.yaml$/
    yaml.safeLoad sh.cat program.args[0]
  when program.args[0] in themesAvailable
    yaml.safeLoad sh.cat pathUtil.join(themesDir, program.args[0])
  else
    exitWithError "Theme must be a valid json or yaml file or a builtin theme"

# throw an error when a config file is specified and not found
if program.config? and not fileExists program.config
  exitWithError "Config file not found: #{program.config}"

# get the contents of the config file specified on the command line, or look in
# the default locations for the config file
HOME = process.env.HOME
config = switch
  when program.config
    sh.cat program.config
  when HOME and fileExists "#{HOME}/.i3/config"
    sh.cat "#{HOME}/.i3/config"
  when HOME and fileExists "#{HOME}/.config/i3/config"
    sh.cat "#{HOME}/.config/i3/config"
  else
    exitWithError "Could not find a valid i3 config file"

# do the heavy lifting
config = mkConfig theme, config

# no output file specified. echo result to stdout and we are done.
unless program.output
  sh.echo config
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

# finally write the file to the specified location
fs.writeFile program.output, config, (err) ->
  exitWithError "Could not write to file: #{program.output}\n\n#{err}" if err
