# Individual Project 2: Rust CLI Binary with SQLite
This Rust project implements the Command Line Interaction with the database [book.db](https://github.com/nogibjj/ids706_IndividualProject2_ll442/blob/main/book.db) stored in SQLite

## Environments

* Works with both AWS CodeCatalyst and GitHub Codespaces


## Requirements
- Rust 1.73
- Virtual environment (optional but recommended, already set up as env in Makefile)
- Packages listed in [dependencies] in `Cargo.toml`


## Set Up 
* This part specifies **how to install the packages and run the code**
1. Run `make install` to install all packages listed in Cargo.toml

2. Run `make build-up` to build up Rust code

3. Run `make test` to test the project

4. Run `make refactor` to check up the style and format of the Rust Code

5. Run `make run` to check the functionality of the database query in ./src/database.py

(You can also Run `make all` to finish all the set up operations at the same time)

# Project Requirement
## 1. Database Interaction CRUD
After set up, you can run `make deploy` to get the output of database query in ./src/main.py

## 2. Use of GitHub Copilot

## 3. Optimized Rust Binary

## 4. Structure
(Project Requirement)
1. `./src`: **Rust source code** with CRUD queries

2. `README.md`: A file that clearly explains what the project does, its dependencies, how to run the program, and how GitHub Copilot was used.

3. `.github\workflows`: GitHub Actions. A workflow file that tests, builds, and lints your Rust code.

(Other files)
4. `.devcontainer` includes a Dockerfile, devcontainer.json. The files in this container specify how the project can be set up.

5. `Cargo.toml` includes all configuration, dependencies and metadata of Rust Project.

6. `Makefile` includes all the make settings for this project

7. `book.db` is the database we will operate on

8. `.gitignore` includes all the files that do not want to be tracked by github

## 5. Video Demo
Attached is the explanation of individual project. Through this video, you can better understand how it works!:D
