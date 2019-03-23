import unittest
from pathlib import Path

from syntect import *


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_can_highlight_lines(self):
        ss = SyntaxSet.load_defaults_nonewlines()
        ts = ThemeSet.load_defaults()
        syntax = ss.find_syntax_by_extension("rs")
        h = HighlightLines(syntax, ts.theme("base16-ocean.dark"))

        ranges = h.highlight("pub struct Wow { hi: u64 }", ss)
        self.assertTrue(len(ranges) > 4)

    def test_can_highlight_file(self):
        testdata_path = Path(__file__).parent / "../../data/testdata"
        ss = SyntaxSet.load_defaults_nonewlines()
        ts = ThemeSet.load_defaults()
        path = testdata_path / "highlight_test.erb"
        syntax = ss.find_syntax_for_file(str(path))
        h = HighlightLines(syntax, ts.theme("base16-ocean.dark"))

        for line in path.read_text().splitlines(True):
            h.highlight(line, ss)

    def test_can_find_regions(self):
        ss = SyntaxSet.load_defaults_nonewlines()
        state = ParseState(ss.find_syntax_by_extension("rb"))
        line = "lol =5+2"
        ops = state.parse_line(line, ss)
        stack = ScopeStack()
        token_count = 0

        for s, op in ScopeRegionIterator(ops, line):
            stack.apply(op)
            if not s:
                continue

            if token_count == 1:
                self.assertEqual(
                    str(stack),
                    str(ScopeStack.from_str("source.ruby keyword.operator.assignment.ruby"))
                )
                self.assertEqual(s, "=")
            token_count += 1
            print(f"{s} {stack}")

        self.assertEqual(token_count, 5)

    def test_can_find_regions_with_trailing_newline(self):
        ss = SyntaxSet.load_defaults_newlines()
        state = ParseState(ss.find_syntax_by_extension("rb"))
        lines = ["# hello world\n", "lol=5+2\n"]
        stack = ScopeStack()

        for line in lines:
            ops = state.parse_line(line, ss)
            print(ops)

            iterated_ops = []
            for _, op in ScopeRegionIterator(ops, line):
                stack.apply(op)
                iterated_ops.append(op)
                print(f"{op}")

            all_ops = [v[1] for v in ops]
            self.assertEqual(len(all_ops), len(iterated_ops) - 1)  # -1 because we want to ignore the NOOP


if __name__ == "__main__":
    unittest.main()
