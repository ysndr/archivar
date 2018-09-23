# archivar

the trackkeeper of your stuff

[![Build
Status](https://travis-ci.org/ysndr/archivar.svg?branch=develop)](https://travis-ci.org/ysndr/archivar)

**`archivar`** is a tool for creating structured folders while keeping
the directory clean using an archive.

## features

  - create a structured archivar manged dir
  - create new projects inside
  - use templates to create recurring structures
  - archive/unarchive projects to keep the directory clean

## planned features

  - git support
  - list projects

## usage

**`archivar`** `[FLAGS] [OPTIONS] <SUBCOMMAND>`

  - **FLAGS**
    
    ``` 
      -h, --help         Prints help information
      -V, --version      Prints version information
      -v, --verbosity    Switches on verbosity (increase verbosity by applying multiple times)
    ```

  - **OPTIONS**
    
    ``` 
      -p, --path <path>    The basedir of the archive [default: .]    
    ```

  - **SUBCOMMANDS**
    
      - `archive <path>`: Move project at `<path>` into archive
      - `init`: Initializes a new archovar managed folder in the base
        dir
      - `new <path> [template]`: Creates new project in `<path>`,
        applies template if `[template]` is given
      - `unarchive <path>`: Restores project at `<path>` from archive

## installation

***Attention***: This application is not (yet) on crates.io

You can still install `archivar` by running

    $ cargo install --git https://github.com/ysndr/archivar

## License

Licensed under the [MIT License](LICENSE-MIT).
