# Altr: Smart CLI Refactoring Tool

## Description
Altr, a command-line tool, specializes in seamless code refactoring while preserving your chosen case styling.

## Usage
Unlock the potential of Altr with the command:
```bash
altr <termToReplace> <replacementTerm> -f <path>
```
## Example
Consider a file, `user.js`, where the "user" needs be renamed to "myImportantUser"
```js
const user = new User("Adam");

function getUserName(user: User) {
    return user.name;
}

// Setting new name for the USER
function setUserName(name: string) {
    user.name = name; 
}
```
```bash
altr user myImportantUser -f user.js
```
```js
const myImportantUser = new MyImportantUser("Adam");

function getMyImportantUserName(myImportantUser: MyImportantUser) {
    return myImportantUser.name;
}

// Setting new name for the MY_IMPORTANT_USER
function setMyImportantUserName(name: string) {
    myImportantUser.name = name; 
}
```

Note that Altr changed the values while keeping into account the casing styles of both the pattern to be replaced and the pattern specified to be renamed
If instead we were to run
```bash
altr user my-important-user -f user.js
```
```js
const my-important-user = new MyImportantUser("Adam");

function getMyImportantUserName(my-important-user: MyImportantUser) {
    return my-important-user.name;
}

// Setting new name for the MY-IMPORTANT-USER
function setMyImportantUserName(name: string) {
    my-important-user.name = name; 
}
```
Note that the casing of the term "user" is not sufficient to determine its conversion into "my-important-user", hence the replacementTerm's casing is taken into account. Although, "User" can be determined to be PascalCase, hence that is correctly inferred

## Installation
Install using cargo
```bash
cargo install altr
```