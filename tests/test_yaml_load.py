import textwrap
import unittest

from syntect import *


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_can_parse(self):
        defn = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: C
            scope: source.c
            contexts: {main: []}
        """), False, None)
        self.assertEqual(defn.name, "C")
        self.assertEqual(defn.scope, Scope("source.c"))
        self.assertEqual(defn.file_extensions, [])
        self.assertEqual(defn.hidden, False)
        self.assertEqual(defn.variables, {})

        defn2 = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: C
            scope: source.c
            file_extensions: [c, h]
            hidden: true
            variables:
              ident: '[QY]+'
            contexts:
              prototype:
                - match: lol
                  scope: source.php
              main:
                - match: \\b(if|else|for|while|{{ident}})\\b
                  scope: keyword.control.c keyword.looping.c
                  captures:
                      1: meta.preprocessor.c++
                      2: keyword.control.include.c++
                  push: [string, 'scope:source.c#main', 'CSS.sublime-syntax#rule-list-body']
                  with_prototype:
                    - match: wow
                      pop: true
                - match: '\"'
                  push: string
              string:
                - meta_scope: string.quoted.double.c
                - meta_include_prototype: false
                - match: \\\\.
                  scope: constant.character.escape.c
                - match: '\"'
                  pop: true
        """), False, None)
        self.assertEqual(defn2.name, "C")
        top_level_scope = Scope("source.c")
        self.assertEqual(defn2.scope, top_level_scope)
        self.assertEqual(defn2.file_extensions, ["c", "h"])
        self.assertEqual(defn2.hidden, True)
        self.assertEqual(defn2.variables["ident"], "[QY]+")

        # main = defn2.contexts["main"]
        # self.assertEqual(main.meta_content_scope, [top_level_scope])
        # self.assertEqual(main.meta_scope, n)
        # self.assertEqual(main.meta_include_prototype, True)

    def test_can_parse_embed_as_with_prototypes(self):
        SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: C
            scope: source.c
            file_extensions: [c, h]
            variables:
              ident: '[QY]+'
            contexts:
              main:
                - match: '(>)\s*'
                  captures:
                    1: meta.tag.style.begin.html punctuation.definition.tag.end.html
                  push:
                    - [{ meta_include_prototype: false }, { meta_content_scope: 'source.css.embedded.html' }, { match: '(?i)(?=</style)', pop: true }]
                    - scope:source.css
                  with_prototype:
                    - match: (?=(?i)(?=</style))
                      pop: true
        """), False, None)

        SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: C
            scope: source.c
            file_extensions: [c, h]
            variables:
              ident: '[QY]+'
            contexts:
              main:
                - match: '(>)\s*'
                  captures:
                    1: meta.tag.style.begin.html punctuation.definition.tag.end.html
                  embed: scope:source.css
                  embed_scope: source.css.embedded.html
                  escape: (?i)(?=</style)
        """), False, None)

        # self.assertEqual(old_def.contexts["main"], def_with_embed.contexts["main"])

    def test_errors_on_embed_without_escape(self):
        pass

    def test_errors_on_regex_compile_error(self):
        pass

    def test_can_parse_ugly_yaml(self):
        defn = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: LaTeX
            scope: text.tex.latex
            contexts:
              main:
                - match: '((\\\\)(?:framebox|makebox))\\b'
                  captures:
                    1: support.function.box.latex
                    2: punctuation.definition.backslash.latex
                  push:
                    - [{meta_scope: meta.function.box.latex}, {match: '', pop: true}]
                    - argument
                    - optional-arguments
              argument:
                - match: '\\{'
                  scope: punctuation.definition.group.brace.begin.latex
                - match: '(?=\\S)'
                  pop: true
              optional-arguments:
                - match: '(?=\\S)'
                  pop: true
        """), False, None)
        self.assertEqual(defn.name, "LaTeX")
        self.assertEqual(defn.hidden, False)
        self.assertEqual(defn.file_extensions, [])

    def test_names_anonymous_contexts(self):
        pass

    def test_can_use_fallback_name(self):
        defn = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            scope: source.c
            contexts:
              main:
                - match: ''
        """), False, None)
        self.assertNotEqual(defn.name, "C")

    def test_can_rewrite_regex(self):
        pass

    def test_can_get_valid_captures_from_regex(self):
        pass

    def test_can_get_valid_captures_from_regex2(self):
        pass

    def test_can_get_valid_captures_from_nested_regex(self):
        pass


if __name__ == "__main__":
    unittest.main()
