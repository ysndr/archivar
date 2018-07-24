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

      \usepackage[ttdefault=true]{AnonymousPro}
      \renewcommand\familydefault{\ttdefault}

babel-newcommands: |
      \addto\captionsngerman{
        \renewcommand{\figurename}{Abb.}%
        \renewcommand{\tablename}{Tab.}%
      }
...


# Archivar

### :: USAGE:
    archivar [FLAGS] <SUBCOMMAND>

### :: FLAGS:
    -v               Increases verbosity
    -h, --help       Prints help information
    -V, --version    Prints version information

### :: SUBCOMMANDS:
    archive      archive project
    help         Prints this message or the help of the given subcommand(s)
    init         command to execute
    new          create new project
    unarchive    unarchive project

### :: archive

- USAGE:
`archivar` **`archive`** `[FLAGS]` `[OPTIONS]` `<PATH>`

- FLAGS:





kjlkjlkjlkjljlkjlk

![Here is my caption, with *emphasis* etc.](https://avatars0.githubusercontent.com/u/3044?s=88&v=4)
