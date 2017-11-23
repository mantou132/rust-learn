const fs = require('fs');
const child_process = require('child_process');
const readline = require('readline');
const watch = require('node-watch');

const prompt = '> ';
watch('./', { recursive: true}, function(evt, name) {
  const match = name.match(/^(\w+)\/src\/.*\.rs/);
  if (match) {
    process.stdout.write('\r');
    console.log('%s changed.', name);
    console.log(child_process.execSync(`cd ${match[1]} && cargo run`).toString('utf8'));
    process.stdout.write(prompt);
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
    try {
      console.log(child_process.execSync(input).toString('utf8'));
    } catch (e) {
      console.log('\x1b[31m');
      console.log(e);
      console.log('\x1b[0m');
    }
  }
  process.stdout.write(prompt);
});

process.stdout.write(prompt);
