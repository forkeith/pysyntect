import sys
import textwrap
from pathlib import Path

from syntect import *

if __name__ == '__main__':
    testdata = Path(__file__).parent / "../data/st_build_3149/themes"
    ss = SyntaxSet.load_defaults_newlines()
    theme = ThemeSet.get_theme(str(testdata / "monokai.tmTheme"))
    settings = theme.settings
    c = settings.background
    path = sys.argv[1]

    content = '\n'.join(highlighted_html_for_file(path, ss, theme).splitlines())
    print(textwrap.dedent(f"""\
        <head>
            <title>{path}</title>
            <style>
                pre {{
                    font-size:13px;
                    font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
                }}
            </style>
        </head>
        <body style=\"background-color:#{c.r:x}{c.g:x}{c.b:x}\">\n
    {content}
        </body>
    """))
