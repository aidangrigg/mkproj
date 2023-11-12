# mkproj

mkproj is a simple command line utility for instantiating projects. 

## Installation

### Using cargo

1. Clone the repository using `git clone https://github.com/aidangrigg/mkproj.git`
2. Cd into the cloned project and run `cargo install --path .` (NOTE: make sure `$HOME/.cargo/bin` is on you path)

## Usage

### Creating templates

Firstly, you must create a directory containing project templates on your system. A template is a directory containing the files needed to instantiate a new project. Below is an example of a template directory containing a cpp & rust template:

```
    .
    ├── cpp
    │   ├── CMakeLists.txt
    │   ├── flake.nix
    │   └── src
    │       └── main.cpp
    └── rust
        ├── Cargo.toml
        ├── flake.nix
        └── src
            └── main.rs
```

Inside these templates, you may define project specific variables inside the template files using the following syntax `$*{variable_name}*`. These variables will be replaced when instantiating your project (use these variables for things like the project name).

### Creating a project

You can call mkproj using the following command `mkproj <PROJECT-DIR>`. mkproj will ask for a few things before instantiating your project.

1. It will ask for the path to your template directory. After being given this path, it will ask if you would like to save this template directory to a configuration file (so you don't have to write your template directory every time you call the command).

2. Next it will ask which template to use. Enter the template you want to use for the project you are instantiating.

3. Finally, it will ask for the names for the project variables you would like to use.

After this, your new project will be instantiated using the selected template, with the project variables replaced with the values you gave it.
