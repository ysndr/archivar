---
title: Archivar Documentation
subtitle: Proof of Concept
author: Yannik Sander ([ysndr](https://github.com/ysndr), [<me@ysndr.de>](mailto:me@ysndr.de))

geometry: "a4paper, top=1.5cm, bottom=1.5cm, left=2.5cm, right=2cm"
header-includes:
    - \usepackage{sectsty}
    - \paragraphfont{\normalfont\ttfamily}
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
      \newcommand{\command}[1]{ \paragraph{#1}\mbox{} }




...

# :: commands

\command{\bf{archivar \it{init}} \ul{path} [-{}-git]}

initializes a new archivar managed directory \ul{path}

1. `mkdir` if needed
#. if git enabled: run `git init`
#. `mkdir` empty `.template/` folder
#. `mkdir` empty `archive/` folder
#. create `.archivar` config/flag file
#. if git enabled commit new archive



\command{\bf{archivar \it{new}} \ul{path} [-d|-{}-dir \ul{ARCHIVAR\_ROOT}] [-t|-{}-template \ul{tpath} [\ul{..template-options}]]}

i. \bf{fail!} if
    - \code{\ul{ARCHIVAR\_ROOT}} is set but does not exist
    - \ul{path} is subfolder of \code{\ul{ARCHIVAR\_ROOT}/archive/}
    - \code{\ul{ARCHIVAR\_ROOT}/\ul{path}} exists
ii. \bf{warn!} if
    - \code{\ul{tpath}} set but  \code{\ul{ARCHIVAR\_ROOT}/.templates/\ul{tpath}} does not exist
1. \code{mkdir \ul{ARCHIVAR\_ROOT}/\ul{path}}
#. create \code{\ul{ARCHIVAR\_ROOT}/\ul{path}/.archived} config/flag file
#. if \code{\ul{tpath}} given  and found initialize template in target directory see section **[:: templating]**
#. if git enabled
    - stage all files
    - write commit



\command{\bf{archivar \it{archive}} \ul{path} [-d|-{}-dir \ul{ARCHIVAR\_ROOT}] [-{}-no-commit]}

archives project

i. __fail!__ if
    - \code{\ul{ARCHIVAR\_ROOT}} is set but does not exist
    - \ul{path} is subfolder of \code{\ul{ARCHIVAR\_ROOT}/archive/}
    - \code{\ul{ARCHIVAR\_ROOT}/\ul{path}} does not exists
1. (git) move project to archive
    - \code{\ul{ARCHIVAR\_ROOT}/ul{path}} -> \code{\ul{ARCHIVAR\_ROOT}/archive/ul{path}}
#. change access rights of archived project to \code{\bf{ro}}
    4. keep executables executable if set so
    4. commit changes if not inhibited by flag \code{-{}-no-commit} or not in git managed \code{\ul{ARCHIVAR\_ROOT}}




\command{\bf{archivar \it{unarchive}} \ul{path} [-d|-{}-dir ARCHIVAR\_ROOT] [-{}-no-commit]}

unarchives project

1. __fail!__ if
    - \code{\ul{ARCHIVAR\_ROOT}} is set but does not exist
    - \ul{path} is not subfolder of \code{\ul{ARCHIVAR\_ROOT}/archive/} or does not exist[TODO: check neccessary]
2. (git) move project from archive
        - \code{\ul{ARCHIVAR\_ROOT}/archive/ul{path}} -> \code{\ul{ARCHIVAR\_ROOT}/ul{path}}
3. change access rights of archived project to \code{\bf{rw}}
    4. keep executables executable if set so
4. commit changes if not inhibited by flag \code{-{}-no-commit} or not in git managed \code{\ul{ARCHIVAR\_ROOT}}


-----


# :: global options

\command{[-d|-{}-dir \ul{ARCHIVAR\_ROOT}]}
- set archivars working dir all \ul{path}s are resolved relative to this
- defaults to `.`


-----


# :: templating
a template is a folder under \code{\ul{ARCHIVAR\_ROOT}/.templates} which contains an \code{\it{.template}} descriptor file

## :: templating :: descriptor file
