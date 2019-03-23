import textwrap

from syntect import *


class Stats:

    def __init__(self):
        self.selectors = {
            "comment": ScopeSelector.from_str("comment - comment.block.attribute"),
            "doc_comment": ScopeSelectors.from_str("comment.line.documentation, comment.block.documentation"),
            "function": ScopeSelector.from_str("entity.name.function"),
            "types": ScopeSelectors.from_str("entity.name.class, entity.name.struct, entity.name.enum, entity.name.type"),
        }
        self.files = 0
        self.functions = 0
        self.types = 0
        self.lines = 0
        self.chars = 0
        self.code_lines = 0
        self.comment_lines = 0
        self.comment_chars = 0
        self.comment_words = 0
        self.doc_comment_lines = 0
        self.doc_comment_words = 0

    def __str__(self):
        # Blank lines (lines-blank-comment):    {stats.lines-stats.code_lines-stats.comment_lines}

        return textwrap.dedent(f"""\
            ################## Stats ###################
            File count:                           {self.files}
            Total characters:                     {self.chars}

            Function count:                       {self.functions}
            Type count (structs, enums, classes): {self.types}

            Code lines (traditional SLOC):        {self.code_lines}
            Total lines (w/ comments & blanks):   {self.lines}
            Comment lines (comment but no code):  {self.comment_lines}
            Blank lines (lines-blank-comment):    {self.lines-self.code_lines-self.comment_lines}

            Lines with a documentation comment:   {self.doc_comment_lines}
            Total words written in doc comments:  {self.doc_comment_words}
            Total words written in all comments:  {self.comment_words}
            Characters of comment:                {self.comment_chars}
        """)


def count_line(ops, line, stack, stats):
    stats.lines += 1

    line_has_comment = False
    line_has_doc_comment = False
    line_has_code = False

    for s, op in ScopeRegionIterator(ops, line):
        stack.apply(op)
        if not s:
            continue

        if stats.selectors["comment"].does_match(stack):
            words = s.split(" ")
            words = [w for w in words if all(c for c in w if c.isalpha() or c in [".", "'"])]
            num_words = len(words)

            if stats.selectors["doc_comment"].does_match(stack):
                line_has_doc_comment = True
                stats.doc_comment_words += num_words
            stats.comment_chars += len(s)
            stats.comment_words += num_words
            line_has_comment = True
        elif s.strip():
            line_has_code = True

        if stats.selectors["function"].does_match(stack):
            stats.functions += 1
        if stats.selectors["types"].does_match(stack):
            stats.types += 1

    if line_has_comment and not line_has_code:
        stats.comment_lines += 1
    if line_has_doc_comment:
        stats.doc_comment_lines += 1
    if line_has_code:
        stats.code_lines += 1


def count(ss, path, stats):
    syntax = ss.find_syntax_for_file(path)
    stats.files += 1
    state = ParseState(syntax)
    stack = ScopeStack()

    text = Path(path).read_text()
    for num_line, line in enumerate(text.splitlines(True)):
        ops = state.parse_line(line, ss)
        stats.chars += len(line)
        count_line(ops, line, stack, stats)


if __name__ == '__main__':
    from pathlib import Path
    ss = SyntaxSet.load_defaults_newlines()
    path = Path(__file__).parent

    stats = Stats()
    print("################## Files ###################")
    for p in Path(path).glob("**/*.py"):
        print(p)
        count(ss, str(p), stats)

    print(stats)
