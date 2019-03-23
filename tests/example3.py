from terminaltables import AsciiTable

from syntect import *  # noqa

table_data = [
    ['Symbol']
]
table_data += [[v] for v in dir() if not v.startswith("__") and "table" not in v.lower()]
table = AsciiTable(table_data)


print(table.table)
