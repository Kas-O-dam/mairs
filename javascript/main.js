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
        // begin[0] -= 1;
        // begin[1] -= 1;
        // end[0] += 1;
        // end[1] += 1;
        let deltaX = end[0] - begin[0];
        let deltaY = end[1] - begin[1];
        function buildByY(){
            let points = [];
            let koef = deltaX/deltaY;
            for(let coorY = begin[1]; coorY < end[1] + 1; coorY++){
                let coorX = Math.round(coorY * koef) + begin[0];
                if((coorX < lawyer.length) && (coorY < lawyer[0].length) && (deltaY != 0)){
                    console.log("Y: ", [coorX, coorY])
                    points.push([coorX, coorY]);
                }else{
                    console.log("IndexInfo: A line will builded by cycle via abscissa");
                    buildByX();
                    return
                };
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                console.log("build")
                lawyer[points[coors][0]][points[coors][1]].char = char
            };
        };
        function buildByX(){
            let points = [];
            let koef = deltaY/deltaX;
            for(let coorX = begin[0]; coorX < end[0] + 1; coorX++){
                let coorY = Math.round(coorX * koef) + begin[1];
                if(coorX < lawyer.length && coorY < lawyer[0].length){
                    console.log("X: ", [coorX, coorY])
                    points.push([coorX, coorY]);
                }else{
                    console.log("IndexError: I can't build the line, because maybe coordinates are uncorrect");
                };
                for(let coors = 0; coors < points.length - 1; coors++){
                    lawyer[points[coors][0]][points[coors][1]].char = char
                };
            };
        };
        buildByY();
        return lawyer;
    }
};

let f = new Field(10, 10, "#");
f.matrix = f.line([3, 3], [7, 5], "0");
let str = f.print("");
console.log(str);