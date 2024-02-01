# Altr: Smart CLI Refactoring Tool

## Overview
Altr is a robust command-line refactoring tool designed to effortlessly enhance your codebase while maintaining your preferred casing styles.

## How to Use
Execute Altr with the following command:
```bash
altr <termToReplace> <replacementTerm> -f <path>
```

## Example
Let's take a look at a practical scenario using a file named `programmer.js`, where we want to rename "programmer" to "rustProgrammer".
```js
const programmer = new Programmer(Adam);

function getProgrammerName(programmer: Programmer) {
    return programmer.name;
}

// Updating the name for the PROGRAMMER
function setProgrammerName(name: string) {
    programmer.name = name; 
}
```
Executing the command:
```bash
altr programmer rustProgrammer -f programmer.js
```
Results in the following updated file:
```js
const rustProgrammer = new RustProgrammer(Adam);

function getRustProgrammerName(rustProgrammer: RustProgrammer) {
    return rustProgrammer.name;
}

// Updating the name for the RUST_PROGRAMMER
function setRustProgrammerName(name: string) {
    rustProgrammer.name = name; 
}
```

Altr intelligently handles the replacement, considering the casing styles of both the original term and the specified replacement term. For instance, if we run:
```bash
altr programmer rust_programmer -f programmer.js
```
The tool adjusts the file accordingly:
```js
const rust_programmer = new RustProgrammer(Adam);

function getRustProgrammerName(rust_programmer: RustProgrammer) {
    return rust_programmer.name;
}

// Updating the name for the RUST_PROGRAMMER
function setRustProgrammerName(name: string) {
    rust_programmer.name = name; 
}
```
Notice how Altr adapts to the casing requirements of the replacement term.

## IO Support
Altr supports taking in input from stdin and passing it to stdout
```bash
$ echo "programmer" | altr programmer rust_programmer
$ rust_programmer
```
You can also specify the output location
```bash
altr programmer rust_programmer -f programmer.js -o rust-programmer.js
```
You can use "-" as path to indicate stdin or stdout as well

## Installation
Altr is easily installed using cargo:
```bash
cargo install altr
```
