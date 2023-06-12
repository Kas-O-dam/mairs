from field import Field

f = Field(50, 10)
f.matrix = f.line((3, 8), (30, 3), '0')
#f.matrix = f.line((0, 0), (49, 9), '*')
f.matrix[3][8].char = '+'
f.matrix[30][3].char = '+'
f.print()