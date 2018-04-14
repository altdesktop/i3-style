files = [
  'lib'
  'src'
]

fs = require 'fs'
{print} = require 'util'
{spawn, exec} = require 'child_process'

try
  which = require('which').sync
catch err
  if process.platform.match(/^win/)?
    console.log 'WARNING: the which module is required for windows\ntry: npm install which'
  which = null

bold = '\x1b[0;1m'
green = '\x1b[0;32m'
reset = '\x1b[0m'
red = '\x1b[0;31m'

task 'docs', 'generate documentation', -> docco()

task 'build', 'compile source', -> build -> log ":)", green

task 'watch', 'compile and watch', -> build true, -> log ":-)", green

task 'test', 'run tests', -> build -> mocha -> log ":)", green

task 'clean', 'clean generated files', -> clean -> log ";)", green


walk = (dir, done) ->
  results = []
  fs.readdir dir, (err, list) ->
    return done(err, []) if err
    pending = list.length
    return done(null, results) unless pending
    for name in list
      file = "#{dir}/#{name}"
      try
        stat = fs.statSync file
      catch err
        stat = null
      if stat?.isDirectory()
        walk file, (err, res) ->
          results.push name for name in res
          done(null, results) unless --pending
      else
        results.push file
        done(null, results) unless --pending

log = (message, color, explanation) -> console.log color + message + reset + ' ' + (explanation or '')

launch = (cmd, options=[], callback) ->
  cmd = which(cmd) if which
  app = spawn cmd, options
  app.stdout.pipe(process.stdout)
  app.stderr.pipe(process.stderr)
  app.on 'exit', (status) -> callback?() if status is 0

build = (watch, callback) ->
  if typeof watch is 'function'
    callback = watch
    watch = false

  options = ['-c', '-b', '-o' ]
  options = options.concat files
  options.unshift '-w' if watch
  launch 'coffee', options, ->
    cliText = fs.readFileSync('./lib/cli.js').toString()
    cliText = "#!/usr/bin/env node\n#{cliText}"
    fs.writeFile './lib/cli.js', cliText, (err) ->
      throw err if err?
      fs.chmod './lib/cli.js', '0755', callback

unlinkIfCoffeeFile = (file) ->
  if file.match /\.coffee$/
    fs.unlink file.replace('src','lib').replace(/\.coffee$/, '.js'), ->
    true
  else false

clean = (callback) ->
  try
    for file in files
      unless unlinkIfCoffeeFile file
        walk file, (err, results) ->
          for f in results
            unlinkIfCoffeeFile f

    callback?()
  catch err

moduleExists = (name) ->
  try 
    require name 
  catch err 
    log "#{name} required: npm install #{name}", red
    false


mocha = (options, callback) ->
  #if moduleExists('mocha')
  if typeof options is 'function'
    callback = options
    options = []
  # add coffee directive
  options.push '--compilers'
  options.push 'coffee:coffee-script/register'
  options.push '-C'
  
  launch 'mocha', options, callback

docco = (callback) ->
  #if moduleExists('docco')
  walk 'src', (err, files) -> launch 'docco', files, callback

