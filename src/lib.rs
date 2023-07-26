#[derive(Debug)]

pub struct Field{
   pub seq: Vec<Vec<Vec<char>>>,
   pub x: usize,
   pub y: usize,
   pub default_char: char,
}
struct Layer {
   matrix: Vec<Vec<char>>,
   x: usize,
   y: usize
}
impl Field{
   pub fn build_layer(&mut self) -> Vec<Vec<char>>{
      let mut layer: Vec<Vec<char>> = Vec::new();
      for _step_x in 0..self.x {
         let empty_vec: Vec<char> = Vec::new();
         layer.push(empty_vec);
         for _step_y in 0..self.y {
            layer[_step_x].push(self.default_char);
         };
      };
      layer
   }
   pub fn unite(&self, layers:&Vec<Vec<Vec<char>>>) -> Vec<Vec<char>>{
      //println!("{:?}", layers);
      let mut layer_result: Vec<Vec<char>> = layers[0].clone();
      //println!("{:?}", layer_result);
      for x in 1..layers.len() {
         for y in 0..layers[x].len() {
            for tile in 0..layers[x][y].len() {
               if layers[x][y][tile] != self.default_char {
                  layer_result[y][tile] = layers[x][y][tile];
               };
               //println!("l: {}, x: {}, y: {}, c: {}, is space: {}", x, y, tile, layers[x][y][tile], layers[x][y][tile] == self.default_char);
            };
         };
      };
      //println!("{:?}", layer_result);
      layer_result
   }
   pub fn print(&self){
      let all_layers = self.unite(&self.seq);

      for y in 0..self.y {
         println!("");
         for x in 0..self.x {
            print!("{}", all_layers[x][y]);
         };
      };
   }
   pub fn print_sepchar(&self, sepchar:char){
      let all_layers = self.unite(&self.seq);

      for y in 0..self.y {
         println!("");
         for x in 0..self.x {
            print!("{}{}", all_layers[x][y], sepchar);
         };
      };
   }
   pub fn clone(&self, layer:&Vec<Vec<char>>) -> Vec<Vec<char>>{
      let mut new_layer: Vec<Vec<char>> = Vec::new();
      for step_x in 0..layer.len() {
         new_layer.push(Vec::new());
         let last_vec = new_layer.len() - 1;
         for step_y in 0..layer[0].len() {
            new_layer[last_vec].push(layer[step_x][step_y]);
         };
      };
      new_layer
   }
   pub fn paste(&self, layer:&Vec<Vec<char>>, slice:Vec<Vec<char>>, begin:[i32; 2]) -> Vec<Vec<char>>{
      let mut new_layer: Vec<Vec<char>> = self.clone(layer);
      for step_x in begin[0] as usize..slice.len() {
         for step_y in begin[1] as usize..slice[0].len() {
            new_layer[step_x][step_y] = slice[step_x][step_y];
         };
      };
      new_layer
   }
   pub fn cut(&self, layer:&Vec<Vec<char>>, begin:[i32; 2], end:[i32; 2]) -> (Vec<Vec<char>>, Vec<Vec<char>>){
      let mut slice: Vec<Vec<char>> = Vec::new();
      let mut new_layer: Vec<Vec<char>> = self.clone(layer);
      for step_x in begin[0]..end[0] - 1 {
         slice.push(Vec::new());
         let last_slice_row: usize = slice.len() - 1;
         for step_y in begin[1]..end[1] - 1 {
            slice[last_slice_row].push(layer[step_x as usize][step_y as usize]);
            new_layer[step_x as usize][step_y as usize] = self.default_char;
         };
      };
      (new_layer, slice)
   }
   pub fn copy(&self, layer:&Vec<Vec<char>>, begin:[i32; 2], end:[i32; 2]) -> Vec<Vec<char>>{
      let mut slice: Vec<Vec<char>> = Vec::new();
      for step_x in begin[0]..end[0] - 1 {
         slice.push(Vec::new());
         let last_slice_row: usize = slice.len() - 1;
         for step_y in begin[1]..end[1] - 1 {
            slice[last_slice_row].push(layer[step_x as usize][step_y as usize]);
         };
      };
      slice
   }
   // GEOMETRY
   pub fn round(&mut self, center:[i32; 2], radius:i32, char:char) -> Vec<Vec<char>>{
      let mut layer = self.build_layer();
      let a = [center[0], center[1] - radius];
      let b = [center[0] + radius, center[1]];
      let c = [center[0], center[1] + radius];
      let d = [center[0] - radius, center[1]];
      let mut  mover = a;
      layer[a[0] as usize][a[1] as usize] = char;
      layer[b[0] as usize][b[1] as usize] = char;
      layer[c[0] as usize][c[1] as usize] = char;
      layer[d[0] as usize][d[1] as usize] = char;
      while mover[0] != b[0] || mover[1] != b[1] {
         let b1 = [mover[0] + 1, mover[1]];
         let b2 = [mover[0], mover[1] + 1];
         let b3 = [mover[0] + 1, mover[1] + 1];
         //console.debug(b1, b2, b3); //
         let delta1 = [(center[0] - b1[0]).abs(), (center[1] - b1[1]).abs()];
         let delta2 = [(center[0] - b2[0]).abs(), (center[1] - b2[1]).abs()];
         let delta3 = [(center[0] - b3[0]).abs(), (center[1] - b3[1]).abs()];
         //console.debug(delta1, delta2, delta3); //
         let hipotenuse1 = ((delta1[0].pow(2) + delta1[1].pow(2)) as f32).sqrt() as i32;
         let hipotenuse2 = ((delta2[0].pow(2) + delta2[1].pow(2)) as f32).sqrt() as i32;
         let hipotenuse3 = ((delta3[0].pow(2) + delta3[1].pow(2)) as f32).sqrt() as i32;
         
         let list = [hipotenuse1, hipotenuse2, hipotenuse3];
         let mut min = radius.pow(2);
         for i in 0..list.len() {
            if (list[i] - radius).abs() < (min - radius).abs() {
               min = list[i]};
         };
         if min == hipotenuse1 { mover = b1 };
         if min == hipotenuse2 { mover = b2 };
         if min == hipotenuse3 { mover = b3 };
         //console.debug(mover, list); //
         layer[mover[0] as usize][mover[1] as usize] = char;
      };
      while mover[0] != c[0] || mover[1] != c[1] {
         let c1 = [mover[0] - 1, mover[1]];
         let c2 = [mover[0], mover[1] + 1];
         let c3 = [mover[0] - 1, mover[1] + 1];
         //console.debug(c1, c2, c3); //
         let delta1 = [(center[0] - c1[0]).abs(), (center[1] - c1[1]).abs()];
         let delta2 = [(center[0] - c2[0]).abs(), (center[1] - c2[1]).abs()];
         let delta3 = [(center[0] - c3[0]).abs(), (center[1] - c3[1]).abs()];
         //console.debug(delta1, delta2, delta3); //
         let hipotenuse1 = ((delta1[0].pow(2) + delta1[1].pow(2)) as f32).sqrt() as i32;
         let hipotenuse2 = ((delta2[0].pow(2) + delta2[1].pow(2)) as f32).sqrt() as i32;
         let hipotenuse3 = ((delta3[0].pow(2) + delta3[1].pow(2)) as f32).sqrt() as i32;
         
         let list = [hipotenuse1, hipotenuse2, hipotenuse3];
         let mut  min = radius.pow(2);
         for i in 0..list.len() {
            if (list[i] - radius).abs() < (min - radius).abs() {
               min = list[i]};
         };
         if min == hipotenuse1 { mover = c1 };
         if min == hipotenuse2 { mover = c2 };
         if min == hipotenuse3 { mover = c3 };
         //console.debug(mover, list); //
         layer[mover[0] as usize][mover[1] as usize] = char;
      };
      while mover[0] != d[0] || mover[1] != d[1] {
          let d1 = [mover[0] - 1, mover[1]];
          let d2 = [mover[0], mover[1] - 1];
          let d3 = [mover[0] - 1, mover[1] - 1];
          //console.debug(d1, d2, d3); //
          let delta1 = [(center[0] - d1[0]).abs(), (center[1] - d1[1]).abs()];
          let delta2 = [(center[0] - d2[0]).abs(), (center[1] - d2[1]).abs()];
          let delta3 = [(center[0] - d3[0]).abs(), (center[1] - d3[1]).abs()];
          //console.debug(delta1, delta2, delta3); //
          let hipotenuse1 = ((delta1[0].pow(2) + delta1[1].pow(2)) as f32).sqrt() as i32;
          let hipotenuse2 = ((delta2[0].pow(2) + delta2[1].pow(2)) as f32).sqrt() as i32;
          let hipotenuse3 = ((delta3[0].pow(2) + delta3[1].pow(2)) as f32).sqrt() as i32;
          
          let list = [hipotenuse1, hipotenuse2, hipotenuse3];
          let mut min = radius.pow(2);
          for i in 0..list.len() {
            if (list[i] - radius).abs() < (min - radius).abs() {
               min = list[i]};
         };
          if min == hipotenuse1 { mover = d1 };
          if min == hipotenuse2 { mover = d2 };
          if min == hipotenuse3 { mover = d3 };
          //console.debug(mover, list); //
          layer[mover[0] as usize][mover[1] as usize] = char;
      };
      while mover[0] != a[0] || mover[1] != a[1] {
          let a1 = [mover[0] + 1, mover[1]];
          let a2 = [mover[0], mover[1] - 1];
          let a3 = [mover[0] + 1, mover[1] - 1];
          //console.debug(a1, a2, a3); //
          let delta1 = [(center[0] - a1[0]).abs(), (center[1] - a1[1]).abs()];
          let delta2 = [(center[0] - a2[0]).abs(), (center[1] - a2[1]).abs()];
          let delta3 = [(center[0] - a3[0]).abs(), (center[1] - a3[1]).abs()];
          //console.debug(delta1, delta2, delta3); //
          let hipotenuse1 = ((delta1[0].pow(2) + delta1[1].pow(2)) as f32).sqrt() as i32;
          let hipotenuse2 = ((delta2[0].pow(2) + delta2[1].pow(2)) as f32).sqrt() as i32;
          let hipotenuse3 = ((delta3[0].pow(2) + delta3[1].pow(2)) as f32).sqrt() as i32;
          
          let list = [hipotenuse1, hipotenuse2, hipotenuse3];
          let mut min = radius.pow(2);
          for i in 0..list.len() {
            if (list[i] - radius).abs() < (min - radius).abs() {
               min = list[i]};
         };
          if min == hipotenuse1 { mover = a1 };
          if min == hipotenuse2 { mover = a2 };
          if min == hipotenuse3 { mover = a3 };
          //console.debug(mover, list); //
          layer[mover[0] as usize][mover[1] as usize] = char;
      };
      return layer;
  }
   /*
       ___     _____    ____    _______
      |  _ \  | ____|  / ,-._\ |__   __|
      | '-'/  | |---, / /         | |
      | |\ \  | ,---' \ \____     | |
      |_| \_\ |_____|  \____/     |_|
   */
   pub fn rect(&mut self, begin:[usize; 2], end:[usize; 2], sym:char) -> Vec<Vec<char>>{
      let mut layer: Vec<Vec<char>> = self.build_layer();
      let mut cycle_begin_x = begin[0];
      let mut cycle_end_x = end[0];
      let mut cycle_begin_y = begin[1];
      let mut cycle_end_y = end[1];
      if begin[0] > end[0] {
         cycle_begin_x = end[0];
         cycle_end_x = begin[0];
      };
      if begin[1] > end[1] {
         cycle_begin_y = end[1];
         cycle_end_y = begin[1];
      };
      for step_x in cycle_begin_x..cycle_end_x + 1 {
         for step_y in cycle_begin_y..cycle_end_y + 1 {
            layer[step_x][step_y] = sym;
         };
      };
      layer
   }
   /*
   	 _        _    _    _    ______
		| |      | |  | \_ | |  |  ____|
		| |      | |  | |\\| |  | |----,
		| |____  | |  | | \\ |  | |----'
		|_____|  |_|  |_|  \_|  |______|
   */
   pub fn line(&mut self, begin:[i32; 2], end:[i32; 2], sym:char) -> Vec<Vec<char>>{
      let mut layer: Vec<Vec<char>> = self.build_layer();
      if (begin[0] - end[0]).abs() >= (begin[1]-end[1]).abs(){
         println!("build by x"); //debug
         let mut points: Vec<[usize; 2]> = Vec::new();
         let mut koef_y: i32 = 1;
         let mut cycle_begin: i32 = begin[0];
         let mut cycle_end: i32 = end[0] + 1;
         let mut begin_y: i32 = begin[1];
         let mut error: f32 = 0.0;
         let corner_koef: f32 = ((end[1]-begin[1]).abs() as f32)/((end[0]-begin[0]).abs() as f32);
         if end[1] < begin[1]{
            koef_y = -1
         };
         if begin[0] > end[0]{
            cycle_begin = end[0];
            cycle_end = begin[0] + 1;
            begin_y = end[1];
            koef_y = -koef_y;
         };
         let mut y: i32 = begin_y;
         for x in cycle_begin..cycle_end{
            if error >= 0.5{
               y += koef_y;
               error -= 1.0;
            };
            points.push([x as usize, y as usize]);
            //println!("x: {}  y: {}  e: {}  ck: {}", x, y, error, corner_koef); //debug
            error += corner_koef;
         };
         for coors in &points{
            layer[coors[0]][coors[1]] = sym;
         };
      }else{
         println!("build by y"); //debug
         let mut points: Vec<[usize; 2]> = Vec::new();
         let mut koef_x: i32 = 1;
         let mut cycle_begin: i32 = begin[1];
         let mut cycle_end: i32 = end[1] + 1;
         let mut begin_x: i32 = begin[0];
         let mut error: f32 = 0.0;
         let corner_koef: f32 = ((end[0]-begin[0]).abs() as f32)/((end[1]-begin[1]).abs() as f32);
         if end[0] < begin[0]{
            koef_x = -1
         };
         if begin[1] > end[1]{
            cycle_begin = end[1];
            cycle_end = begin[1] + 1;
            begin_x = end[0];
            koef_x = -koef_x;
         };
         let mut x: i32 = begin_x;
         for y in cycle_begin..cycle_end{
            if error >= 0.5{
               x += koef_x;
               error -= 1.0;
            };
            points.push([x as usize, y as usize]);
            // println!("x: {}  y: {}  e: {}  ck: {}", x, y, error, corner_koef); //debug
            error += corner_koef;
         };
         for coors in &points{
            layer[coors[0]][coors[1]] = sym;
         };
      };
      layer
   }
}
fn main(){
   const X: usize = 40;
   const Y: usize = 20;
   let mut layer = Field{
      seq: Vec::new(),
      x: X,
      y: Y,
      default_char: '.',
   };

   // "exams" for functions
   
   //let mut new_line = layer.line([5, 4], [9, 4], '0');
   //layer.seq.push(new_line); // a
   //layer.seq.push(layer.line([9, 4], [5, 4], '1')); // a reverse
   //layer.seq.push(layer.line([5, 4], [9, 2], '0')); // b
   //layer.seq.push(layer.line([9, 2], [5, 4], '1')); // b reverse
   //layer.seq.push(layer.line([5, 4], [9, 0], '0')); // c
   //layer.seq.push(layer.line([9, 0], [5, 4], '1')); // c reverse
   //new_line = layer.line([5, 4], [7, 0], '+');
   //layer.seq.push(new_line); // d
   //layer.seq.push(layer.line([7, 0], [5, 4], '1')); // d reverse
   //layer.seq.push(layer.line([5, 4], [5, 0], '0')); // e
   //layer.seq.push(layer.line([5, 0], [5, 4], '1')); // e reverse
   //let peace = layer.cut(&layer.seq[0], [3, 3], [8, 8]);
   //println!("{:#?}", layer.paste(&layer.seq[0], peace.1, [3, 3]));
   //println!("{:#?}", layer.copy(&layer.seq[0], [3, 3], [8, 8]));
   //let new_layer = layer.rect([7, 8], [3, 3], '0');
   //layer.seq.push(new_layer);
   //let new_round = layer.round([20, 10], 8, '0');
   //layer.seq.push(new_round);
   //layer.print_sepchar('|');
}
