const fs = require('fs');
const child_process = require('child_process');
const readline = require('readline');
const watch = require('node-watch');

const prompt = '> ';

const exec = command => {
  child_process.exec(command, (error, stdout, stderr) => {
    if (error) {
      stdout && console.log(stdout);
      console.log('\x1b[31m');
      console.log(stderr);
      console.log('\x1b[0m');
    } else {
      console.log(stdout);
    }
    process.stdout.write(prompt);
  });
};

let timer;
watch('./', { recursive: true}, function(evt, name) {
  const match = name.match(/^(\w+)[\/\\](src|tests)[\/\\](.*)\.rs/);
  if (match) {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      process.stdout.write('\r');
      exec(`cd ${match[1]} && cargo ${
        match[2] === 'tests' || match[3] === 'lib' ? 'test' : 'run'
      }`);
    }, 100);
  }
});

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  prompt,
  terminal : true,
});

rl.on('line', (input) => {
  if (input) {
    exec(input);
  }
});

process.stdout.write(prompt);
