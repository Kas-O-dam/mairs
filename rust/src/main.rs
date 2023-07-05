#[derive(Debug)]

struct Field{
   seq: Vec<Vec<Vec<char>>>,
   x: usize,
   y: usize,
   default_char: char,
}
impl Field{
   fn build_layer(&mut self) -> Vec<Vec<char>>{
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
   fn unite(&self, layers:&Vec<Vec<Vec<char>>>) -> Vec<Vec<char>>{
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
   fn print(&self){
      let all_layers = self.unite(&self.seq);
      for y in 0..self.y {
         println!("");
         for x in 0..self.x {
            print!("{}", all_layers[x][y]);
         };
      };
   }
   fn clone(&self, layer:&Vec<Vec<char>>) -> Vec<Vec<char>>{
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
   fn paste(&self, layer:&Vec<Vec<char>>, slice:Vec<Vec<char>>, begin:[i32; 2]) -> Vec<Vec<char>>{
      let mut new_layer: Vec<Vec<char>> = self.clone(layer);
      for step_x in begin[0] as usize..slice.len() {
         for step_y in begin[1] as usize..slice[0].len() {
            new_layer[step_x][step_y] = slice[step_x][step_y];
         };
      };
      new_layer
   }
   fn cut(&self, layer:&Vec<Vec<char>>, begin:[i32; 2], end:[i32; 2]) -> (Vec<Vec<char>>, Vec<Vec<char>>){
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
   fn copy(&self, layer:&Vec<Vec<char>>, begin:[i32; 2], end:[i32; 2]) -> Vec<Vec<char>>{
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
   /*
       ___     _____    ____    _______
      |  _ \  | ____|  / ,-._\ |__   __|
      | '-'/  | |---, / /         | |
      | |\ \  | ,---' \ \____     | |
      |_| \_\ |_____|  \____/     |_|
   */
   fn rect(&mut self, begin:[usize; 2], end:[usize; 2], sym:char) -> Vec<Vec<char>>{
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
   fn line(&mut self, begin:[i32; 2], end:[i32; 2], sym:char) -> Vec<Vec<char>>{
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
   const X: usize = 10;
   const Y: usize = 10;
   let mut layer = Field{
      seq: Vec::new(),
      x: X,
      y: Y,
      default_char: '.',
   };

   // "exams" for functions
   
   let mut new_line = layer.line([5, 4], [9, 4], '0');
   layer.seq.push(new_line); // a
   //layer.seq.push(layer.line([9, 4], [5, 4], '1')); // a reverse
   //layer.seq.push(layer.line([5, 4], [9, 2], '0')); // b
   //layer.seq.push(layer.line([9, 2], [5, 4], '1')); // b reverse
   //layer.seq.push(layer.line([5, 4], [9, 0], '0')); // c
   //layer.seq.push(layer.line([9, 0], [5, 4], '1')); // c reverse
   new_line = layer.line([5, 4], [7, 0], '+');
   layer.seq.push(new_line); // d
   //layer.seq.push(layer.line([7, 0], [5, 4], '1')); // d reverse
   //layer.seq.push(layer.line([5, 4], [5, 0], '0')); // e
   //layer.seq.push(layer.line([5, 0], [5, 4], '1')); // e reverse
   //let peace = layer.cut(&layer.seq[0], [3, 3], [8, 8]);
   //println!("{:#?}", layer.paste(&layer.seq[0], peace.1, [3, 3]));
   //println!("{:#?}", layer.copy(&layer.seq[0], [3, 3], [8, 8]));
   //let new_layer = layer.rect([7, 8], [3, 3], '0');
   //layer.seq.push(new_layer);
   layer.print();
}