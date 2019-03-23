import textwrap
import unittest
from pathlib import Path

from syntect import *


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_can_load(self):
        packages_path = Path(__file__).parent / "../../data/testdata/Packages"
        builder = SyntaxSetBuilder()
        builder.add_from_folder(str(packages_path.resolve()), False)

        # cmake_dummy_syntax = SyntaxDefinition(
        #     name= "CMake",
        #     file_extensions= ["CMakeLists.txt", "cmake"],
        #     scope= Scope("source.cmake"),
        #     first_line_match= None,
        #     hidden= False,
        #     variables= {},
        #     contexts= {},
        # )

        # builder.add(cmake_dummy_syntax)
        builder.add_plain_text_syntax()

        ps = builder.build()

        self.assertEqual(ps.find_syntax_by_first_line("#!/usr/bin/env node").name, "JavaScript")
        rails_scope = Scope("source.ruby.rails")
        syntax = ps.find_syntax_by_name("Ruby on Rails")
        # ps.find_syntax_plain_text()
        self.assertEqual(ps.find_syntax_by_extension("rake").name, "Ruby")
        self.assertEqual(ps.find_syntax_by_token("ruby").name, "Ruby")
        self.assertEqual(ps.find_syntax_by_first_line("lol -*- Mode: C -*- such line").name, "C")
        self.assertEqual(ps.find_syntax_for_file("testdata/parser.rs").name, "Rust")
        # self.assertEqual(ps.find_syntax_for_file("testdata/test_first_line.test")
        #                .expect("Error finding syntax for file")
        #                .expect("No syntax found for file")
        #                .name,
        #            "Ruby")
        self.assertEqual(ps.find_syntax_for_file(".bashrc").name, "Bourne Again Shell (bash)")
        # self.assertEqual(ps.find_syntax_for_file("CMakeLists.txt").name, "CMake")
        # self.assertEqual(ps.find_syntax_for_file("test.cmake").name, "CMake")
        self.assertEqual(ps.find_syntax_for_file("Rakefile").name, "Ruby")
        self.assertRaises(Exception, ps.find_syntax_by_first_line, "derp derp hi lol")
        self.assertEqual(ps.find_syntax_by_path("Packages/Rust/Rust.sublime-syntax").name, "Rust")
        self.assertEqual(syntax.scope, rails_scope)
        # let main_context = ps.get_context(&syntax.contexts["main"])
        # let count = syntax_definition::context_iter(&ps, main_context).count()
        # self.assertEqual(count, 109)

    def test_can_clone(self):
        syntax_a = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: A
            scope: source.a
            file_extensions: [a]
            contexts:
              main:
                - match: 'a'
                  scope: a
                - match: 'go_b'
                  push: scope:source.b#main
        """), True)

        syntax_b = SyntaxDefinition.load_from_str(textwrap.dedent("""\
            name: B
            scope: source.b
            file_extensions: [b]
            contexts:
              main:
                - match: 'b'
                  scope: b
        """), True)

        builder = SyntaxSetBuilder()
        builder.add(syntax_a)
        builder.add(syntax_b)
        builder.build()


if __name__ == "__main__":
    unittest.main()
