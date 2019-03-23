import unittest
from pathlib import Path

from syntect import *


def rgba(r, g, b, a):
    return r << 24 | g << 16 | b << 8 | a


class Test(unittest.TestCase):

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_can_parse_common_themes(self):
        testdata_path = Path(__file__).parent / "../../data/testdata"
        ts = ThemeSet.load_from_folder(str(testdata_path))
        all_themes = [v for v in ts.names]
        print(all_themes)
        self.assertTrue("base16-ocean.dark" in all_themes)

        theme = ThemeSet.get_theme(str(testdata_path / "spacegray/base16-ocean.dark.tmTheme"))
        self.assertEqual(theme.name, "Base16 Ocean Dark")
        self.assertEqual(
            theme.settings.selection,
            Color(0x4f, 0x5b, 0x66, 0xff)
        )
        self.assertEqual(
            theme.scopes[0].style.foreground,
            Color(0xc0, 0xc5, 0xce, 0xFF)
        )


if __name__ == "__main__":
    unittest.main()
