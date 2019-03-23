from syntect import *  # noqa


def does_match(scope_name, selector):
    try:
        return ScopeSelector.from_str(selector).does_match(
            ScopeStack.from_str(scope_name)
        )
    except Exception as e:
        return None


if __name__ == "__main__":
    lst = [
        "keyword.control.php", "keyword",
        "keyword.control.php", "keyword.control",
        "keyword.control.php", "control",
        "keyword.control.php", "keyword.cont",
        "keyword.control.php", "keyword.control.php.embedded",

        "source.php meta.block.php keyword.control.php", "keyword",
        "source.php meta.block.php keyword.control.php", "meta",
        "source.php meta.block.php keyword.control.php", "keyword meta",

        "source.php meta.block.php", "text | meta",
        "source.php", "text, meta",

        "source.php meta.block.php keyword.control.php", "keyword & meta",
        "source.php meta.block.php", "keyword & meta",

        "source.php meta.block.php", "source - keyword",
        "source.php meta.block.php keyword.control.php", "source - keyword",

        "source.php meta.block.php", "source - (keyword | storage)",
        "source.php meta.block.php", "(source - source.php) | text",
    ]

    for scope_name, selector in zip(lst[::2], lst[1::2]):
        print("{:<50}{:<50}{}".format(
            scope_name, selector, does_match(scope_name, selector))
        )
