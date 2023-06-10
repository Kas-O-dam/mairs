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
		deltaX = end[0] - begin[0]
		deltaY = end[1] - begin[1]
		def buildByY():
			points = []
			koef = deltaX/deltaY
			for coorY in range(begin[1], end[1]+1):
				print(coorY*koef+begin[0], coorY)
				coorX = round(coorY*koef)+begin[0]
				if coorX < len(lawyer) and coorY < len(lawyer[0]):
					points.append((coorX, coorY))
				else:
					raise IndexError()
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		def buildByX():
			points = []
			koef = deltaY/deltaX
			for coorX in range(begin[0], end[0]+1):
				print(coorX, coorX*koef+begin[1])
				coorY = round(coorX*koef)+begin[1]
				if coorX < len(lawyer) and coorY < len(lawyer[0]):
					points.append((coorX, coorY))
				else:
					raise IndexError()
			for coors in points:
				lawyer[coors[0]][coors[1]].char = char
		try:
			buildByX()
		except IndexError:
			buildByY()
		except ZeroDivisionError:
			buildByY()
		return lawyer