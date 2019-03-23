from syntect import *  # noqa

table = [
    [0, '#', (0, 16), "# I'm a comment\n",
     "source.python comment.line.number-sign.python punctuation.definition.comment.python "],
    [1, ' ', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [2, 'I', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [3, "'", (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [4, 'm', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [5, ' ', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [6, 'a', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [7, ' ', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [8, 'c', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [9, 'o', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [10, 'm', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [11, 'm', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [12, 'e', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [13, 'n', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [14, 't', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [15, '\n', (0, 16), "# I'm a comment\n", "source.python comment.line.number-sign.python "],
    [16, '\n', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [17, 'd', (17, 24), 'def foo', "source.python meta.function.python storage.type.function.python "],
    [18, 'e', (17, 24), 'def foo', "source.python meta.function.python storage.type.function.python "],
    [19, 'f', (17, 24), 'def foo', "source.python meta.function.python storage.type.function.python "],
    [20, ' ', (17, 24), 'def foo', "source.python meta.function.python "],
    [21, 'f', (17, 24), 'def foo', "source.python meta.function.python entity.name.function.python meta.generic-name.python "],
    [22, 'o', (17, 24), 'def foo', "source.python meta.function.python entity.name.function.python meta.generic-name.python "],
    [23, 'o', (17, 24), 'def foo', "source.python meta.function.python entity.name.function.python meta.generic-name.python "],
    [24, '(', (24, 26), '()', "source.python meta.function.parameters.python punctuation.section.parameters.begin.python "],
    [25, ')', (24, 26), '()', "source.python meta.function.parameters.python punctuation.section.parameters.end.python "],
    [26, ':', (25, 27), '):', "source.python meta.function.python punctuation.section.function.begin.python "],
    [27, '\n', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [28, ' ', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [29, ' ', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [30, ' ', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [31, ' ', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
    [32, 'p', (32, 38), 'print(',
     "source.python meta.function-call.python meta.qualified-name.python support.function.builtin.python "],
    [33, 'r', (32, 38), 'print(',
     "source.python meta.function-call.python meta.qualified-name.python support.function.builtin.python "],
    [34, 'i', (32, 38), 'print(',
     "source.python meta.function-call.python meta.qualified-name.python support.function.builtin.python "],
    [35, 'n', (32, 38), 'print(',
     "source.python meta.function-call.python meta.qualified-name.python support.function.builtin.python "],
    [36, 't', (32, 38), 'print(',
     "source.python meta.function-call.python meta.qualified-name.python support.function.builtin.python "],
    [37, '(', (37, 38), '(', "source.python meta.function-call.python punctuation.section.arguments.begin.python "],
    [38, "'", (38, 52), "'# No comment'", "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python punctuation.definition.string.begin.python "],
    [39, '#', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [40, ' ', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [41, 'N', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [42, 'o', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [43, ' ', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [44, 'c', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [45, 'o', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [46, 'm', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [47, 'm', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [48, 'e', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [49, 'n', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [50, 't', (38, 52), "'# No comment'",
     "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python "],
    [51, "'", (38, 52), "'# No comment'", "source.python meta.function-call.python meta.function-call.arguments.python meta.string.python string.quoted.single.python punctuation.definition.string.end.python "],
    [52, ')', (51, 53), "')", "source.python meta.function-call.python punctuation.section.arguments.end.python "],
    [53, '\n', (0, 54), "# I'm a comment\n\ndef foo():\n    print('# No comment')\n", "source.python "],
]


def extract_scope(scopes, i):
    length = len(scopes)
    base = scopes[i]

    # a
    b = i
    for s2 in scopes[i + 1::1]:
        s1 = min([base, s2], key=len)
        s2 = max([base, s2], key=len)
        if not s2.startswith(s1):
            break
        b += 1

    # b
    a = i
    for s2 in scopes[i - 1::-1]:
        s1 = min([base, s2], key=len)
        s2 = max([base, s2], key=len)
        if not s2.startswith(s1):
            break
        a -= 1

    return (max(0, a), min(b, length))


def extract_regions(data):
    lst = [list(map(operator.itemgetter(1), g)) for k, g in groupby(enumerate(data), lambda v:v[0] - v[1])]
    return [(v[0], v[-1]) for v in lst]


def test():
    from collections import defaultdict

    scopes_positions = defaultdict(list)

    for i, v in enumerate(table):
        scopes_positions[v[4]].append(i)

    regions = {}
    lst = []
    for k, v in scopes_positions.items():
        for r in extract_regions(v):
            regions[r] = k
            lst.append(r)

    lst = sorted(lst)
    for v in lst:
        print(v, regions[v])


if __name__ == '__main__':
    scopes = [v[4] for v in table]

    for i, v in enumerate(table):
        r1 = v[2]
        r2 = extract_scope(scopes, i)
        print("{:<5}{:<10}{:<10}{}".format(
            i, f"({r1[0]},{r1[1]})", f"({r2[0]},{r2[1]})", "OK" if r1 == r2 else "FAILED"
        ))
