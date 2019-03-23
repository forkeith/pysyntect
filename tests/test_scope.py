import unittest

from syntect import *


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_repo_works(self):
        repo = ScopeRepository()
        self.assertEqual(repo.build("source.php"), repo.build("source.php"))
        self.assertEqual(repo.build("source.php.wow.hi.bob.troll.clock.5"),
                         repo.build("source.php.wow.hi.bob.troll.clock.5"))
        self.assertEqual(repo.build(""), repo.build(""))
        s1 = repo.build("")
        self.assertEqual(repo.to_string(s1), "")
        s2 = repo.build("source.php.wow")
        self.assertEqual(repo.to_string(s2), "source.php.wow")
        self.assertTrue(repo.build("source.php") != repo.build("source.perl"))
        self.assertTrue(repo.build("source.php") != repo.build("source.php.wagon"))
        self.assertEqual(repo.build("comment.line."), repo.build("comment.line"))

    def test_global_repo_works(self):
        self.assertFalse(Scope("source.php") != Scope("source.php"))
        self.assertTrue(Scope("source.php") == Scope("source.php"))
        try:
            Scope.from_str("1.2.3.4.5.6.7.8")
        except ExceptionType:
            self.fail("from_str shouldn't crash here")
        self.assertRaises(Exception, Scope.from_str, "1.2.3.4.5.6.7.8.9")

    def test_prefixes_work(self):
        self.assertTrue(Scope("1.2.3.4.5.6.7.8").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))
        self.assertTrue(Scope("1.2.3.4.5.6").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))
        self.assertTrue(Scope("1.2.3.4").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))
        self.assertFalse(Scope("1.2.3.4.5.6.a").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))
        self.assertFalse(Scope("1.2.a.4.5.6.7").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))
        self.assertFalse(Scope("1.2.a.4.5.6.7").is_prefix_of(Scope("1.2.3.4.5")))
        self.assertFalse(Scope("1.2.a").is_prefix_of(Scope("1.2.3.4.5.6.7.8")))


if __name__ == "__main__":
    unittest.main()
