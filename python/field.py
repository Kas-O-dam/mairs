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
			cycleBegin = begin[1]
			cycleEnd = end[1] + 1
			beginX = begin[0]
			if end[0] < begin[0]:
				koefX = -1
			else:
				koefX = 1
			if begin[1] > end[1]:
				cycleBegin = end[1]
				cycleEnd = begin[1] + 1
				beginX = end[0]
				koefX = -koefX
			cornerKoef = abs(end[0]-begin[0])/abs(end[1]-begin[1])
			error = 0
			x = beginX
			for y in range(cycleBegin, cycleEnd):
				if error >= 0.5:
					x += koefX
					error -= 1
				points.append((x, y))
				print("x: ", x, " y: ", y, " e: ", error, " ck: ", cornerKoef)
				error += cornerKoef
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		def buildByX():
			print("Build by x")
			points = []
			koefY = 0
			cycleBegin = begin[0]
			cycleEnd = end[0] + 1
			beginY = begin[1]
			if end[1] < begin[1]:
				koefY = -1
			else:
				koefY = 1
			if begin[0] > end[0]:
				cycleBegin = end[0]
				cycleEnd = begin[0] + 1
				beginY = end[1]
				koefY = -koefY
			cornerKoef = abs((end[1]-begin[1])/(end[0]-begin[0]))
			error = 0
			y = beginY
			for x in range(cycleBegin, cycleEnd):
				if error >= 0.5:
					y += koefY
					error -= 1
				points.append((x, y))
				print("x: ", x, " y: ", y, " e: ", error, " ck: ", cornerKoef)
				error += cornerKoef
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		if abs(begin[0]-end[0]) >= abs(begin[1]-end[1]):
			buildByX()
		else:
			buildByY()
		return lawyer