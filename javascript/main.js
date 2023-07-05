let libMode = 'DEBUG' //RELEASE, DEBUG or TURBO DEBUG

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
    constructor(x, y, char, defaultChar=' '){
        this.sequence = [];
        this.x = x;
        this.y = y;
        this.defaultCharacter = defaultChar;
    }
    build(char=null){
        let layer = [];
        for(let i = 0; i < this.x; i++){
            layer.push([]);
            for(let j = 0; j < this.y; j++){
                layer[layer.length - 1].push(new Tile(char, 100));
            };
        };
        return layer;
    }
    unite(sequence){
        let result = this.build();
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
    getSequenceAsString(separate=''){
        let resultAsString = String();
        let allLayers = this.unite(this.sequence);
        // console.log(allLayers); //
        for(let y = 0; y < this.y; y++){
            for(let x = 0; x < this.x; x++){
                if(allLayers[x][y].char != null){
                    resultAsString += allLayers[x][y].char + separate; 
                }else{
                    resultAsString += this.defaultCharacter + separate;
                };
            };
            resultAsString += '\n'
        };
        return resultAsString
    }
    round(center, radius, char=null){
        let layer = this.build();
        let a = new Array(center[0], center[1] - radius);
        let b = new Array(center[0] + radius, center[1]);
        let c = new Array(center[0], center[1] + radius);
        let d = new Array(center[0] - radius, center[1]);
        let mover = a;
        layer[a[0]][a[1]].char = char;
        layer[b[0]][b[1]].char = char;
        layer[c[0]][c[1]].char = char;
        layer[d[0]][d[1]].char = char;
        while(mover[0] != b[0] || mover[1] != b[1]){
            let b1 = new Array(mover[0] + 1, mover[1]);
            let b2 = new Array(mover[0], mover[1] + 1);
            let b3 = new Array(mover[0] + 1, mover[1] + 1);
            //console.debug(b1, b2, b3); //
            let delta1 = new Array(Math.abs(center[0] - b1[0]), Math.abs(center[1] - b1[1]));
            let delta2 = new Array(Math.abs(center[0] - b2[0]), Math.abs(center[1] - b2[1]));
            let delta3 = new Array(Math.abs(center[0] - b3[0]), Math.abs(center[1] - b3[1]));
            //console.debug(delta1, delta2, delta3); //
            let hipotenuse1 = Math.sqrt(delta1[0]**2 + delta1[1]**2);
            let hipotenuse2 = Math.sqrt(delta2[0]**2 + delta2[1]**2);
            let hipotenuse3 = Math.sqrt(delta3[0]**2 + delta3[1]**2);
            
            let list = [hipotenuse1, hipotenuse2, hipotenuse3];
            let min = radius ** 2;
            for(let i = 0; i < list.length; i++){
                if(Math.abs(list[i] - radius) < Math.abs(min - radius)) min = list[i]
            };
            if(min == hipotenuse1) mover = b1;
            if(min == hipotenuse2) mover = b2;
            if(min == hipotenuse3) mover = b3;
            //console.debug(mover, list); //
            layer[mover[0]][mover[1]].char = char;
        };
        while(mover[0] != c[0] || mover[1] != c[1]){
            let c1 = new Array(mover[0] - 1, mover[1]);
            let c2 = new Array(mover[0], mover[1] + 1);
            let c3 = new Array(mover[0] - 1, mover[1] + 1);
            //console.debug(c1, c2, c3); //
            let delta1 = new Array(Math.abs(center[0] - c1[0]), Math.abs(center[1] - c1[1]));
            let delta2 = new Array(Math.abs(center[0] - c2[0]), Math.abs(center[1] - c2[1]));
            let delta3 = new Array(Math.abs(center[0] - c3[0]), Math.abs(center[1] - c3[1]));
            //console.debug(delta1, delta2, delta3); //
            let hipotenuse1 = Math.sqrt(delta1[0]**2 + delta1[1]**2);
            let hipotenuse2 = Math.sqrt(delta2[0]**2 + delta2[1]**2);
            let hipotenuse3 = Math.sqrt(delta3[0]**2 + delta3[1]**2);
            
            let list = [hipotenuse1, hipotenuse2, hipotenuse3];
            let min = radius ** 2;
            for(let i = 0; i < list.length; i++){
                if(Math.abs(list[i] - radius) < Math.abs(min - radius)) min = list[i]
            };
            if(min == hipotenuse1) mover = c1;
            if(min == hipotenuse2) mover = c2;
            if(min == hipotenuse3) mover = c3;
            //console.debug(mover, list); //
            layer[mover[0]][mover[1]].char = char;
        };
        while(mover[0] != d[0] || mover[1] != d[1]){
            let d1 = new Array(mover[0] - 1, mover[1]);
            let d2 = new Array(mover[0], mover[1] - 1);
            let d3 = new Array(mover[0] - 1, mover[1] - 1);
            //console.debug(d1, d2, d3); //
            let delta1 = new Array(Math.abs(center[0] - d1[0]), Math.abs(center[1] - d1[1]));
            let delta2 = new Array(Math.abs(center[0] - d2[0]), Math.abs(center[1] - d2[1]));
            let delta3 = new Array(Math.abs(center[0] - d3[0]), Math.abs(center[1] - d3[1]));
            //console.debug(delta1, delta2, delta3); //
            let hipotenuse1 = Math.sqrt(delta1[0]**2 + delta1[1]**2);
            let hipotenuse2 = Math.sqrt(delta2[0]**2 + delta2[1]**2);
            let hipotenuse3 = Math.sqrt(delta3[0]**2 + delta3[1]**2);
            
            let list = [hipotenuse1, hipotenuse2, hipotenuse3];
            let min = radius ** 2;
            for(let i = 0; i < list.length; i++){
                if(Math.abs(list[i] - radius) < Math.abs(min - radius)) min = list[i]
            };
            if(min == hipotenuse1) mover = d1;
            if(min == hipotenuse2) mover = d2;
            if(min == hipotenuse3) mover = d3;
            //console.debug(mover, list); //
            layer[mover[0]][mover[1]].char = char;
        };
        while(mover[0] != a[0] || mover[1] != a[1]){
            let a1 = new Array(mover[0] + 1, mover[1]);
            let a2 = new Array(mover[0], mover[1] - 1);
            let a3 = new Array(mover[0] + 1, mover[1] - 1);
            //console.debug(a1, a2, a3); //
            let delta1 = new Array(Math.abs(center[0] - a1[0]), Math.abs(center[1] - a1[1]));
            let delta2 = new Array(Math.abs(center[0] - a2[0]), Math.abs(center[1] - a2[1]));
            let delta3 = new Array(Math.abs(center[0] - a3[0]), Math.abs(center[1] - a3[1]));
            //console.debug(delta1, delta2, delta3); //
            let hipotenuse1 = Math.sqrt(delta1[0]**2 + delta1[1]**2);
            let hipotenuse2 = Math.sqrt(delta2[0]**2 + delta2[1]**2);
            let hipotenuse3 = Math.sqrt(delta3[0]**2 + delta3[1]**2);
            
            let list = [hipotenuse1, hipotenuse2, hipotenuse3];
            let min = radius ** 2;
            for(let i = 0; i < list.length; i++){
                if(Math.abs(list[i] - radius) < Math.abs(min - radius)) min = list[i]
            };
            if(min == hipotenuse1) mover = a1;
            if(min == hipotenuse2) mover = a2;
            if(min == hipotenuse3) mover = a3;
            //console.debug(mover, list); //
            layer[mover[0]][mover[1]].char = char;
            console.info(mover[0] != a[0], mover[1] != a[1], a); //
        };
        return layer;
    }
    rect(begin, end, char=this.defaultCharacter){
        let layer = this.build();
        let stepSignX = 1;
        let stepSignY = 1;
        if( end[0] < begin[0] ){
            stepSignX = -1;
        };
        if( end[1] < begin[1] ){
            stepSignY = -1;
        };
        for(let stepX = begin[0] + 1; stepX <= end[0]; stepX += stepSignX){ // don't ask me: 'Why your cycles aren't similar' - if this is working correctly - don't touch to this
            for(let stepY = begin[1]; stepY < end[1]; stepY += stepSignY){
                //console.log( 'x: ', stepX, ' y: ', stepY, 'c: ', char ); //
                layer[stepX][stepY].char = char;
            };
        };
        return layer;
    }
    line(begin, end, char='#'){
        let layer = this.build();
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
				//console.log('x: ', x, ' y: ', y, ' e: ', error); //
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
				//console.log('x: ', x, ' y: ', y, ' e: ', error); //
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

//let f = new Field(50, 50, null);
// calls for testing
//f.sequence.push(f.rect([5, 5], [7, 7], '1'));
//f.sequence.push(f.line([5, 4], [9, 4], '0')); // a
// f.sequence.push(f.line([9, 4], [5, 4], '1')); // a reverse
// f.sequence.push(f.line([5, 4], [9, 2], '0')); // b
// f.sequence.push(f.line([9, 2], [5, 4], '1')); // b reverse
// f.sequence.push(f.line([5, 4], [9, 0], '0')); // c
// f.sequence.push(f.line([9, 0], [5, 4], '1')); // c reverse
// f.sequence.push(f.line([5, 4], [7, 0], '0')); // d
// f.sequence.push(f.line([7, 0], [5, 4], '1')); // d reverse
// f.sequence.push(f.line([5, 4], [5, 0], '0')); // e
// f.sequence.push(f.line([5, 0], [5, 4], '1')); // e reverse
//f.sequence.push(f.round([5, 5], 3, '0'));
//f.sequence[0][5][5].char = '0';
//let str = f.getSequenceAsString('|');
//console.log(str);

//test for custom errors
// new IndexError(' 0,', ' 0', 'layer with index 5');
// new AxesInfo('Layer with index 5', 'x')
// new BuildSuccess('Layer with index 5', [0, 0], [9, 9]);