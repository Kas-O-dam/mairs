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
        this.x = x;
        this.y = y;
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
    consolePrint(separate=""){
        let str = ""
        for(let y = 0; y < this.y; y++){
            for(let x = 0; x < this.x; x++){
                str += this.matrix[x][y].char + separate;
            };
            str += "\n"
        };
        return str
    }
    line(begin, end, char="#"){
        let lawyer = this.matrix;
        function buildByY(){
            let points = [];
            let koefX = 0;
            let cycleBegin = begin[1];
            let cycleEnd = end[1] + 2;
            let beginX = begin[0];
			if(end[0] < begin[0]){
				koefX = -1;
            }else{
				koefX = 1;
            };
            if(begin[1] > end[1]){
                cycleBegin = end[1];
                cycleEnd = begin[1] + 2;
                beginX = end[0];
                koefX = -koefX;
            };
			let cornerKoef = Math.abs(end[0]-begin[0])/Math.abs(end[1]-begin[1]);
			let error = 0;
			let x = beginX;
			for(let y = cycleBegin; y != cycleEnd; y++){
				if(error >= 0.5){
					x += koefX;
					error -= 1;
                };
				points.push([x, y]);
				console.log("x: ", x, " y: ", y, " e: ", error);
				error += cornerKoef
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                lawyer[points[coors][0]][points[coors][1]].char = char
            };
        };
        function buildByX(){
            let points = [];
            let koefY = 0;
            let cycleBegin = begin[0];
            let cycleEnd = end[0] + 2;
            let beginY = begin[1];
			if(end[1] < begin[1]){
				koefY = -1;
            }else{
				koefY = 1;
            };
            if(begin[0] > end[0]){
                cycleBegin = end[0];
                cycleEnd = begin[0] + 2;
                beginY = end[1];
                koefY = -koefY;
            };
			let cornerKoef = Math.abs(end[1]-begin[1])/Math.abs(end[0]-begin[0])
			let error = 0
			let y = beginY
			for(let x = cycleBegin; x != cycleEnd; x++){
				if(error >= 0.5){
					y += koefY;
					error -= 1;
                };
				points.push([x, y]);
				console.log("x: ", x, " y: ", y, " e: ", error);
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

let f = new Field(10, 10, ".");
// calls for testing
// f.matrix = f.line([5, 4], [9, 4], '0') // a
// f.matrix = f.line([9, 4], [5, 4], '1') // a reverse
// f.matrix = f.line([5, 4], [9, 2], '0') // b
// f.matrix = f.line([9, 2], [5, 4], '1') // b reverse
// f.matrix = f.line([5, 4], [9, 0], '0') // c
// f.matrix = f.line([9, 0], [5, 4], '1') // c reverse
// f.matrix = f.line([5, 4], [7, 0], '0') // d
// f.matrix = f.line([7, 0], [5, 4], '1') // d reverse
// f.matrix = f.line([5, 4], [5, 0], '0') // e
f.matrix = f.line([5, 0], [5, 4], '1') // e reverse
let str = f.consolePrint("");
console.log(str);