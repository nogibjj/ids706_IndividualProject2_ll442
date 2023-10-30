# Individual Project 2: Rust CLI Binary with SQLite
This project appears to be a **Rust project** that involves **interacting with a database using CRUD (Create, Read, Update, Delete) operations**. This project also involves using GitHub Copilot, an AI-powered code suggestion tool, during the whole implementation process. Additionally, the project aims to optimize the Rust binary and includes a test suite. 
(Generated by COPILOT)

## Environments
* Works with both AWS CodeCatalyst and GitHub Codespaces


## Requirements
1. Rust 1.73
2. Virtual environment (optional but recommended, already set up as env in Makefile)
3. Packages listed in [dependencies] in `Cargo.toml`, including (Improved with COPILOT)
* `rusqlite`: A Rust library for working with SQLite databases.

* `serde`: A Rust library for serializing and deserializing data structures.

* `serde_json`: A Rust library for working with JSON data.

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
In this project, GitHub Copilot is used in the following ways.
* Generate Project Explanation, How to run the program and Dependencies as required in project instruction

* Trouble shooting and debug

* Code Suggestion

* Generate unit test for the project


## 3. Optimized Rust Binary
* Update workflows: Define how to build up the Rust Project in release mode and then upload the binary as an artifact in "Upload Artifact" part in `./github/workflows/ci_cd.yml`

* Update in Actions: Go to the "Actions" tab on your GitHub repository's web page. Click on the specific workflow run you are interested in. At the bottom of the workflow run summary, you will see an "Artifacts" section. You can download the binary artifact from there.

## 4. Test
In this project, both the **unit tests** and **itegration test** are carried out.

* Unit Tests are small tests that test a specific function or module in isolation. Unit tests in the `lib.rs` file are implemented.

* Integration Tests under `tests/` test the interactions between multiple parts of your library or between your library and other systems.


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
