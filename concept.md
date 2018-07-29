---
title: Archivar Documentation
subtitle: Proof of Concept
author: Yannik Sander ([ysndr](https://github.com/ysndr), [<me@ysndr.de>](mailto:me@ysndr.de))
geometry: "a4paper, top=1.5cm, bottom=1.5cm, left=2.5cm, right=2cm"
lang: de-DE

# mainfont: Hack

header-includes:
    - \usepackage{sectsty}
    - \paragraphfont{\normalfont\ttfamily\bfseries}
    - \subsubsectionfont{\normalfont\ttfamily\bfseries}
    - \newcommand{\hideFromPandoc}[1]{#1}
    - |
      \hideFromPandoc{
        \let\Begin\begin
        \let\End\end
        \let\code\texttt
        \let\it\textit
        \let\bf\textbf
        \let\ul\underline
      }
      \newcommand{\command}[1]{ \paragraph{#1}\mbox{}}

      \usepackage{enumitem}
      \setlist[itemize,1]{label=::}

      
babel-newcommands: |
      \addto\captionsngerman{
        \renewcommand{\figurename}{Abb.}%
        \renewcommand{\tablename}{Tab.}%
      }
...

# Archivar

## Usage

```
archivar 0.1.0
Yannik Sander <me@ysndr.de>
the trachkeeper of your stuff

USAGE:
    archivar [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
        --no-git       disable git integration
    -h, --help         Prints help information
    -V, --version      Prints version information
    -v, --verbosity    switch on verbosity

OPTIONS:
    -p, --path <path>     [default: .]

SUBCOMMANDS:
    archive
    help         Prints this message or the help of the given subcommand(s)
    init
    new
    unarchive
```


## Global Arguments

## Subcommands

### `init`
initializes an archivar managed folder in  archivar path.

(@) *check:* archivar_path is **directory**
(@) *check:* archivar_path is **empty**
(@) *write:* archivar_path/.archivar

### `new`

### `archive`

### `unarchive`
