class IndexInfo{
    constructor(message){
        return message;
    }
};
class IndexError{
    constructor(message){
        return message;
    }
};

class Tile{
    constructor(char, rgba){
        this.char = char;
        this.rgba = rgba;
    }
};

class Field{
    constructor(x, y, char, data){
        this.x = x + 1;
        this.y = y + 1;
        this.matrix = [];
        if(data == undefined){
            for(let i = 0; i < this.x; i++){
                this.matrix.push([]);
                for(let j = 0; j < this.y; j++){
                    this.matrix[this.matrix.length - 1].push(new Tile(char, 100));
                };
            };
        };
    }
    print(separate=""){
        let str = ""
        for(let y = 0; y < this.matrix.length - 1; y++){
            for(let x = 0; x < this.matrix.length - 1; x++){
                str += this.matrix[x][y].char + separate;
            };
            str += "\n"
        };
        return str
    }
    line(begin, end, char="#"){
        let lawyer = this.matrix;
        let deltaX = end[0] - begin[0];
        let deltaY = end[1] - begin[1];
        function buildByY(){
            let points = [];
            let koefX = 0;
			if(end[0] < begin[0]){
				koefX = -1;
            }else{
				koefX = 1;
            };
			let cornerKoef = (end[0]-begin[0])/(end[1]-begin[1])
			let error = 0
			let x = begin[0]
			for(let y = begin[1]; y != end[1] + 1; y++){
				if(error >= 0.5){
					x += koefX;
					error -= 1;
                };
				points.push([x, y]);
				//console.log("x: ", x, " y: ", y, " e: ", error);
				error += cornerKoef
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                lawyer[points[coors][0]][points[coors][1]].char = char
            };
        };
        function buildByX(){
            let points = [];
            let koefY = 0;
			if(end[1] < begin[1]){
				koefY = -1;
            }else{
				koefY = 1;
            };
			let cornerKoef = (end[1]-begin[1])/(end[0]-begin[0])
			let error = 0
			let y = begin[1]
			for(let x = begin[0]; x != end[0] + 1; x++){
				if(error >= 0.5){
					y += koefY;
					error -= 1;
                };
				points.push([x, y]);
				//console.log("x: ", x, " y: ", y, " e: ", error);
				error += cornerKoef
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                lawyer[points[coors][0]][points[coors][1]].char = char
            };
        };
        if(Math.abs(begin[0] - end[0]) > Math.abs(begin[1] - end[1])){
            buildByX();
        }else{
            buildByY();
        };
        return lawyer;
    };
};

let f = new Field(10, 10, "#");
f.matrix = f.line([3, 3], [3, 7], "0");
let str = f.print("");
console.log(str);