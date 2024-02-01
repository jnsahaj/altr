# Altr: Smart CLI Refactoring Tool

## Overview
Altr is a robust command-line refactoring tool designed to effortlessly enhance your codebase while maintaining your preferred casing styles.

## How to Use
Execute Altr with the following command:
```bash
altr <termToReplace> <replacementTerm> -f <path>
```

## Example
Let's take a look at a practical scenario using a file named `user.js`, where we want to rename "user" to "myImportantUser".
```js
const user = new User("Adam");

function getUserName(user: User) {
    return user.name;
}

// Updating the name for the USER
function setUserName(name: string) {
    user.name = name; 
}
```
Executing the command:
```bash
altr user myImportantUser -f user.js
```
Results in the following updated file:
```js
const myImportantUser = new MyImportantUser("Adam");

function getMyImportantUserName(myImportantUser: MyImportantUser) {
    return myImportantUser.name;
}

// Updating the name for the MY_IMPORTANT_USER
function setMyImportantUserName(name: string) {
    myImportantUser.name = name; 
}
```

Altr intelligently handles the replacement, considering the casing styles of both the original term and the specified replacement term. For instance, if we run:
```bash
altr user my-important-user -f user.js
```
The tool adjusts the file accordingly:
```js
const my-important-user = new MyImportantUser("Adam");

function getMyImportantUserName(my-important-user: MyImportantUser) {
    return my-important-user.name;
}

// Updating the name for the MY-IMPORTANT-USER
function setMyImportantUserName(name: string) {
    my-important-user.name = name; 
}
```
Notice how Altr adapts to the casing requirements of the replacement term.

## IO Support
Altr supports taking in input from stdin and passing it to stdout
```bash
$ echo "user" | altr user myImportantUser
$ myImportantUser
```
You can also specify the output location
```bash
altr user myImportantUser -f user.js -o peek.js
```
You can use "-" as path to indicate stdin or stdout as well

## Installation
Altr is easily installed using cargo:
```bash
cargo install altr
```