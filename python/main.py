from field import Field

f = Field(10, 10)
# calls for testing
# f.sequence[0] = f.line((5, 4), (9, 4), '0') # a
# f.sequence.append(f.line((9, 4), (5, 4), '1')) # a reverse
# f.matrix = f.line((5, 4), (9, 2), '0') # b
# f.matrix = f.line((9, 2), (5, 4), '1') # b reverse
# f.matrix = f.line((5, 4), (9, 0), '0') # c
# f.matrix = f.line((9, 0), (5, 4), '1') # c reverse
# f.matrix = f.line((5, 4), (7, 0), '0') # d
# f.matrix = f.line((7, 0), (5, 4), '1') # d reverse
# f.matrix = f.line((0, 0), (49, 9), '*')