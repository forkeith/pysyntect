## pysyntect

[![Freenode](https://img.shields.io/badge/chat-on%20freenode-brightgreen.svg)](https://kiwiirc.com/client/irc.freenode.net/#pyblime)

Python bindings for [syntect](https://github.com/trishume/syntect), a rust
library for syntax highlighting using Sublime Text syntax definitions.

The main purpose why I've created this library was to provide syntax highlighting
to [pyblime](https://github.com/brupelo/pyblime/)

## Docs

- Check out the official [syntect docs](https://docs.rs/syntect)
- At this moment not all syntect structures have been included in pysyntect:

        +-----------------------------+
        | Symbol                      |
        +-----------------------------+
        | Color                       |
        | FontStyle                   |
        | HighlightLines              |
        | MatchPower                  |
        | NOOP                        |
        | ParseState                  |
        | Scope                       |
        | ScopeRegionIterator         |
        | ScopeRepository             |
        | ScopeSelector               |
        | ScopeSelectors              |
        | ScopeStack                  |
        | ScopeStackOp                |
        | Style                       |
        | StyleModifier               |
        | SyntaxDefinition            |
        | SyntaxReference             |
        | SyntaxSet                   |
        | SyntaxSetBuilder            |
        | Theme                       |
        | ThemeItem                   |
        | ThemeSet                    |
        | ThemeSettings               |
        | highlighted_html_for_file   |
        | highlighted_html_for_string |
        +-----------------------------+
