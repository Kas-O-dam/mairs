from tile import Tile
from palette import Palette

class Field:
	def __init__(this,x,y,data=[],char="#"):
		this.sequence = []
		this.x=x+1
		this.y=y+1
		if data==[]:
			this.matrix=[]
			for i in range(0,x+1):
				this.matrix.append([])
				for j in range(0,y+1):
					this.matrix[len(this.matrix)-1].append(Tile(char=char, rgba=Palette[char]))
	def print(this, separate=""):
		for y in range(len(this.matrix[0])-1):
			for x in range(len(this.matrix)-1):
				print(this.matrix[x][y].char + separate, end="")
			print()
	def line(this, begin, end, char="#"):
		lawyer = this.matrix
		def buildByY():
			print("Build by y")
			points = []
			koefX = 0
			if end[0] < begin[0]:
				koefX = -1
			else:
				koefX = 1
			cornerKoef = (end[0]-begin[0])/(end[1]-begin[1])
			error = 0
			x = begin[0]
			for y in range(begin[1], end[1]+1):
				if error >= 0.5:
					x += koefX
					error -= 1
				points.append((x, y))
				print("x: ", x, " y: ", y, " e: ", error)
				error += cornerKoef
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		def buildByX():
			print("Build by x")
			points = []
			koefY = 0
			if end[1] < begin[1]:
				koefY = -1
			else:
				koefY = 1
			cornerKoef = abs((end[1]-begin[1])/(end[0]-begin[0]))
			error = 0
			y = begin[1]
			for x in range(begin[0], end[0]+1):
				if error >= 0.5:
					y += koefY
					error -= 1
				points.append((x, y))
				print("x: ", x, " y: ", y, " e: ", error, " ck: ", cornerKoef)
				error += cornerKoef
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		try:
			buildByX()
		except IndexError:
			buildByY()
		except ZeroDivisionError:
			buildByY()
		return lawyer