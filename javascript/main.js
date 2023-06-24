let libMode = "DEBUG" //RELEASE, DEBUG or TURBO DEBUG

class AxesInfo{
    constructor(who, axes){
        console.info(`[ %cInfo`, 'color: darkgray; font-size: 14px;', `] ${who} will be build via axes ${axes}`);
    }
};
class BuildSuccess{
    constructor(who, begin, end){
        console.info(`[ %cSuccess`, 'color: green; font-size: 14px;', `] ${who} begined from [${begin[0]}, ${begin[1]}] was successfuly build to [${end[0]}, ${end[1]}]`);
    }
};
class IndexError{
    constructor(firstIndex, secondIndex, layer){
        console.error(`[ %cError`, 'color: red; font-size: 14px;', `] Your indexes${firstIndex}${secondIndex} aren't correct in layer: ${layer}`);
    }
};

class Layer{ // this is on future
    constructor(data, name, height, width){
        this.data = data;
        this.name = name;
        this.height = height;
        this.width = width;
    }
};

class Tile{
    constructor(char, rgba){
        this.char = char;
        this.rgba = rgba;
    }
};

class Field{
    constructor(x, y, char, defaultChar=" "){
        this.layerSequence = [];
        this.x = x;
        this.y = y;
        this.defaultCharacter = defaultChar;
    }
    buildLayer(char=null){
        let layer = [];
        for(let i = 0; i < this.x; i++){
            layer.push([]);
            for(let j = 0; j < this.y; j++){
                layer[layer.length - 1].push(new Tile(char, 100));
            };
        };
        return layer;
    }
    uniteLayers(sequence){
        let result = this.buildLayer();
        for(let layer = 0; layer < sequence.length; layer++){
            for(let y = 0; y < this.y; y++){
                for(let x = 0; x < this.x; x++){
                    if(sequence[layer][x][y].char != null){
                        //console.log(sequence[layer][x][y]); //
                        result[x][y].char = sequence[layer][x][y].char
                    }else{
                        continue;
                    };
                };
            };
        };
        return result;
    }
    getSequenceAsJSON(){
        let allLayers = this.uniteLayers(this.layerSequence);
        return JSON.stringify(allLayers);
    }
    getSequenceAsString(separate=""){
        let resultAsString = String();
        let allLayers = this.uniteLayers(this.layerSequence);
        console.log(allLayers); //
        for(let y = 0; y < this.y; y++){
            for(let x = 0; x < this.x; x++){
                if(allLayers[x][y].char != null){
                    resultAsString += allLayers[x][y].char + separate; 
                }else{
                    resultAsString += this.defaultCharacter + separate;
                };
            };
            resultAsString += "\n"
        };
        return resultAsString
    }
    rect(begin, end, char=this.defaultCharacter){
        let layer = this.buildLayer();
        let stepSignX = 1;
        let stepSignY = 1;
        if( end[0] < begin[0] ){
            stepSignX = -1;
        };
        if( end[1] < begin[1] ){
            stepSignY = -1;
        };
        for(let stepX = begin[0] + 1; stepX <= end[0]; stepX += stepSignX){ // don't ask me: "Why your cycles aren't similar" - if this is working correctly - don't touch to this
            for(let stepY = begin[1]; stepY < end[1]; stepY += stepSignY){
                //console.log( "x: ", stepX, " y: ", stepY, "c: ", char ); //
                layer[stepX][stepY].char = char;
            };
        };
        return layer;
    }
    line(begin, end, char="#"){
        let layer = this.buildLayer();
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
				//console.log("x: ", x, " y: ", y, " e: ", error); //
				error += cornerKoef
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                layer[points[coors][0]][points[coors][1]].char = char
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
				//console.log("x: ", x, " y: ", y, " e: ", error); //
				error += cornerKoef
            };
            for(let coors = 0; coors < points.length - 1; coors++){
                layer[points[coors][0]][points[coors][1]].char = char
            };
        };
        if(Math.abs(begin[0] - end[0]) > Math.abs(begin[1] - end[1])){
            buildByX();
        }else{
            buildByY();
        };
        return layer;
    };
};

let f = new Field(10, 10, null);
// calls for testing
f.layerSequence.push(f.rect([5, 5], [7, 7], "1"));
f.layerSequence.push(f.line([5, 4], [9, 4], '0')); // a
// f.layerSequence.push(f.line([9, 4], [5, 4], '1')); // a reverse
// f.layerSequence.push(f.line([5, 4], [9, 2], '0')); // b
// f.layerSequence.push(f.line([9, 2], [5, 4], '1')); // b reverse
// f.layerSequence.push(f.line([5, 4], [9, 0], '0')); // c
// f.layerSequence.push(f.line([9, 0], [5, 4], '1')); // c reverse
// f.layerSequence.push(f.line([5, 4], [7, 0], '0')); // d
// f.layerSequence.push(f.line([7, 0], [5, 4], '1')); // d reverse
// f.layerSequence.push(f.line([5, 4], [5, 0], '0')); // e
// f.layerSequence.push(f.line([5, 0], [5, 4], '1')); // e reverse
let str = f.getSequenceAsString("|");
console.log(str);

//test for custom errors
// new IndexError(" 0,", " 0", "layer with index 5");
// new AxesInfo("Layer with index 5", "x")
// new BuildSuccess("Layer with index 5", [0, 0], [9, 9]);