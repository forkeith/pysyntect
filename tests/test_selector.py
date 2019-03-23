import unittest

from syntect import *


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def scope_selector_does_match(self, s, stack):
        return ScopeSelector.from_str(s).does_match(ScopeStack.from_str(stack))

    def scope_selectors_does_match(self, s, stack):
        return ScopeSelectors.from_str(s).does_match(ScopeStack.from_str(stack))

    def cmp(self, a, b):
        if a:
            a = a.value()
        if b:
            b = b.value()
        self.assertEqual(a, b)

    def test_selectors_work(self):
        sels = ScopeSelectors.from_str("source.php meta.preprocessor - string.quoted, source string")
        self.assertEqual(len(sels), 2)
        first_sel = sels[0]
        self.assertEqual(
            repr(first_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<source.php>, <meta.preprocessor>] }, excludes: [ScopeStack { clear_stack: [], scopes: [<string.quoted>] }] }"
        )

        sels = ScopeSelectors.from_str(
            "text.xml meta.tag.preprocessor.xml punctuation.separator.key-value.xml"
        )
        self.assertEqual(len(sels), 1)
        first_sel = sels[0]
        self.assertEqual(
            repr(first_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<text.xml>, <meta.tag.preprocessor.xml>, <punctuation.separator.key-value.xml>] }, excludes: [] }"
        )

        sels = ScopeSelectors.from_str(
            "text.xml meta.tag.preprocessor.xml punctuation.separator.key-value.xml - text.html - string"
        )
        self.assertEqual(len(sels), 1)
        first_sel = sels[0]
        self.assertEqual(
            repr(first_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<text.xml>, <meta.tag.preprocessor.xml>, <punctuation.separator.key-value.xml>] }, excludes: [ScopeStack { clear_stack: [], scopes: [<text.html>] }, ScopeStack { clear_stack: [], scopes: [<string>] }] }"
        )

        sels = ScopeSelectors.from_str(
            "text.xml meta.tag.preprocessor.xml punctuation.separator.key-value.xml - text.html - string, source - comment"
        )
        self.assertEqual(len(sels), 2)
        first_sel = sels[0]
        self.assertEqual(
            repr(first_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<text.xml>, <meta.tag.preprocessor.xml>, <punctuation.separator.key-value.xml>] }, excludes: [ScopeStack { clear_stack: [], scopes: [<text.html>] }, ScopeStack { clear_stack: [], scopes: [<string>] }] }"
        )
        second_sel = sels[1]
        self.assertEqual(
            repr(second_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<source>] }, excludes: [ScopeStack { clear_stack: [], scopes: [<comment>] }] }"
        )

        sels = ScopeSelectors.from_str(" -a.b|j.g")
        self.assertEqual(len(sels), 2)
        first_sel = sels[0]
        self.assertEqual(
            repr(first_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [] }, excludes: [ScopeStack { clear_stack: [], scopes: [<a.b>] }] }"
        )
        second_sel = sels[1]
        self.assertEqual(
            repr(second_sel),
            "ScopeSelector { path: ScopeStack { clear_stack: [], scopes: [<j.g>] }, excludes: [] }"
        )

    def test_matching_works(self):
        self.cmp(self.scope_selectors_does_match("a.b, a e, e.f", "a.b e.f"), MatchPower(0o20))
        self.cmp(self.scope_selectors_does_match("a.b, a e.f, e.f", "a.b e.f"), MatchPower(0o21))
        self.cmp(self.scope_selectors_does_match("a.b, a e.f - c j, e.f", "a.b c.d j e.f"), MatchPower(0o2000))
        self.cmp(self.scope_selectors_does_match("a.b, a e.f - c j, e.f - a.b", "a.b c.d j e.f"), MatchPower(0o2))
        self.cmp(self.scope_selectors_does_match("a.b, a e.f - c k, e.f - a.b", "a.b c.d j e.f"), MatchPower(0o2001))
        self.cmp(self.scope_selectors_does_match("a.b|a e.f -d, e.f -a.b", "a.b c.d e.f"), MatchPower(0o201))

    def test_empty_stack_matching_works(self):
        self.cmp(self.scope_selector_does_match(" - a.b", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match("", "a.b c.d j e.f"), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match("", ""), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match(" - a.b", ""), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match("a.b - ", ""), None)
        self.cmp(self.scope_selector_does_match("a.b - ", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" - ", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" - a.b", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" - g.h", "a.b c.d j e.f"), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match(" -a.b", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match("", "a.b c.d j e.f"), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match(" -a.b", ""), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match("a.b -", ""), None)
        self.cmp(self.scope_selector_does_match("a.b -", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" -", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" -a.b", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" -g.h", "a.b c.d j e.f"), MatchPower(0o1))

    def test_multiple_excludes_matching_works(self):
        self.cmp(self.scope_selector_does_match(" - a.b - c.d", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" - a.b - c.d", ""), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match("a.b - c.d -e.f", ""), None)
        self.cmp(self.scope_selector_does_match("a.b - c.d -", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selector_does_match(" -g.h - h.i", "a.b c.d j e.f"), MatchPower(0o1))
        self.cmp(self.scope_selector_does_match("a.b", "a.b c.d j e.f"), MatchPower(0o2))
        self.cmp(self.scope_selector_does_match("a.b -g.h - h.i", "a.b c.d j e.f"), MatchPower(0o2))
        self.cmp(self.scope_selector_does_match("c.d", "a.b c.d j e.f"), MatchPower(0o20))
        self.cmp(self.scope_selector_does_match("c.d - j.g - h.i", "a.b c.d j e.f"), MatchPower(0o20))
        self.cmp(self.scope_selectors_does_match("j.g| -a.b", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selectors_does_match(" -a.b|j.g", "a.b c.d j e.f"), None)
        self.cmp(self.scope_selectors_does_match(" -a.b,c.d - j.g - h.i", "a.b c.d j e.f"), MatchPower(0o20))
        self.cmp(self.scope_selectors_does_match(" -a.b, -d.c -f.e", "a.b c.d j e.f"), MatchPower(0o01))


if __name__ == "__main__":
    unittest.main()
