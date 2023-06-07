from field import Field

f = Field(50, 10)
f.matrix = f.line((0, 0), (49, 9), '0')
f.print()