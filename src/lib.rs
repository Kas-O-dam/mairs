extern crate colored;

use colored::Colorize;

#[derive(Debug)]
pub struct Character {
   pub context: char,
   pub red: u8,
   pub green: u8,
   pub blue: u8,
   pub unicode: String
}
pub struct Layer<'layer>{
   pub x: usize,
   pub y: usize,
   pub context: Vec<Vec<&'layer Character>>
}
impl<'a> Layer <'a> {
   pub fn build(&mut self, sym:&'a Character){
      self.context = Vec::with_capacity(self.x);
      for _step_x in 0..self.x {
         let empty_vec: Vec<&Character> = Vec::with_capacity(self.y);
         self.context.push(empty_vec);
         for _step_y in 0..self.y {
            self.context[_step_x].push(sym);
         };
      };
   }
   pub fn copy(&self, begin:[usize; 2], end:[usize; 2]) -> Vec<Vec<&Character>>{
      let mut slice: Vec<Vec<&Character>> = Vec::with_capacity((begin[0] as i32 - end[0] as i32).abs() as usize);
      for step_x in begin[0]..end[0]{
         slice.push(Vec::with_capacity((begin[1] as i32 - end[1] as i32).abs() as usize));
         let last_slice_row: usize = slice.len() - 1;
         for step_y in begin[1]..end[1]{
            slice[last_slice_row].push(self.context[step_x][step_y]);
         };
      };
      slice
   }
   pub fn paste(&mut self, slice:&Vec<Vec<&'a Character>>, begin:[usize; 2]){
      for step_x in 0..slice.len() {
         for step_y in 0..slice[0].len() {
            self.context[begin[0] + step_y][begin[1] + step_x] = slice[step_x][step_y]; //that, what x & y did replaced have only one reason - it is work correctly...
         };
      };
   }
   pub fn cut(&mut self, begin:[usize; 2], end:[usize; 2], sym:&'a Character) -> Vec<Vec<&Character>>{
      let mut slice: Vec<Vec<&Character>> = Vec::new();
      for step_x in begin[0]..end[0] {
         slice.push(Vec::new());
         let last_slice_row: usize = slice.len() - 1;
         for step_y in begin[1]..end[1] {
            slice[last_slice_row].push(self.context[step_x][step_y]);
            self.context[step_x][step_y] = sym;
         };
      };
      slice
   }
   // GEOMETRY
   pub fn horizontal(&mut self, mut x:[usize; 2], y:usize, sym:&'a Character){
      if x[0] > x[1] {(x[0], x[1]) = (x[1], x[0])};
      for step in x[0]..x[1] {
         self.context[step][y] = sym;
      };
   }
   pub fn vertical(&mut self, x:usize, mut y:[usize; 2], sym:&'a Character){
      if y[0] > y[1] {(y[0], y[1]) = (y[1], y[0])};
      for step in y[0]..y[1] {
         self.context[x][step] = sym;
      };
   }
   /*
   	 _        _    _    _    ______
		| |      | |  | \_ | |  |  ____|
		| |      | |  | |\\| |  | |----,
		| |____  | |  | | \\ |  | |----'
		|_____|  |_|  |_|  \_|  |______|
   */
   pub fn line(&mut self, begin:[usize; 2], end:[usize; 2], sym:&'a Character){
      if (begin[0] as i32 - end[0] as i32).abs() >= (begin[1] as i32 - end[1] as i32).abs(){
         //println!("build by x"); //debug
         let mut points: Vec<[usize; 2]> = Vec::new();
         let mut koef_y: i32 = 1;
         let mut cycle_begin: usize = begin[0];
         let mut cycle_end: usize = end[0] + 1;
         let mut begin_y: usize = begin[1];
         let mut error: f32 = 0.0;
         let corner_koef: f32 = ((end[1] as i32 - begin[1] as i32).abs() as f32)/((end[0] as i32 - begin[0] as i32).abs() as f32);
         if end[1] < begin[1]{
            koef_y = -1
         };
         if begin[0] > end[0]{
            cycle_begin = end[0];
            cycle_end = begin[0] + 1;
            begin_y = end[1];
            koef_y = -koef_y;
         };
         let mut y: i32 = begin_y as i32;
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
            self.context[coors[0]][coors[1]] = sym;
         };
      }else{
         //println!("build by y"); //debug
         let mut points: Vec<[usize; 2]> = Vec::new();
         let mut koef_x: i32 = 1;
         let mut cycle_begin: usize = begin[1];
         let mut cycle_end: usize = end[1] + 1;
         let mut begin_x: usize = begin[0];
         let mut error: f32 = 0.0;
         let corner_koef: f32 = ((end[0] as i32 - begin[0] as i32).abs() as f32)/((end[1] as i32 - begin[1] as i32).abs() as f32);
         if end[0] < begin[0]{
            koef_x = -1
         };
         if begin[1] > end[1]{
            cycle_begin = end[1];
            cycle_end = begin[1] + 1;
            begin_x = end[0];
            koef_x = -koef_x;
         };
         let mut x: i32 = begin_x as i32;
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
            self.context[coors[0]][coors[1]] = sym;
         };
      };
   }
   /*
       ___     _____    ____    _______
      |  _ \  | ____|  / ,-._\ |__   __|
      | '-'/  | |---, / /         | |
      | |\ \  | ,---' \ \____     | |
      |_| \_\ |_____|  \____/     |_|
   */
   pub fn rect(&mut self, mut begin:[usize; 2], mut end:[usize; 2], sym:&'a Character){
      //if user want draw reverse rect
      if begin[0] > end[0] { (begin[0], end[0]) = (end[0], begin[0]) };
      if begin[1] > end[1] { (begin[1], end[1]) = (end[1], begin[1]) };
      // without ones, function doesn't working. DON'T TOUCH TO THAT!
      self.horizontal([begin[0], end[0] - 1], begin[1], sym);
      self.horizontal([end[0], begin[0]], end[1] - 1, sym);
      self.vertical(begin[0], [begin[1], end[1]], sym);
      self.vertical(end[0] - 1, [end[1], begin[1]], sym);
}
   /*
       ___     ____    _    _   _    _   __
      |  _ \  / __ \  | |  | | | \  | | |  _',
      | '-'/ | /  \ | | |  | | | \\ | | | | \ \
      | |\ \ | \__/ | | \__/ | | |\\| | | |_/ /
      |_| \_\ \____/   \____/  |_| \__| |__,-'
   */
  pub fn round(&mut self, center:[i32; 2], radius:i32, sym:&'a Character){
   let a = [center[0], center[1] - radius];
   let b = [center[0] + radius, center[1]];
   let c = [center[0], center[1] + radius];
   let d = [center[0] - radius, center[1]];
   let mut  mover = a;
   self.context[a[0] as usize][a[1] as usize] = sym;
   self.context[b[0] as usize][b[1] as usize] = sym;
   self.context[c[0] as usize][c[1] as usize] = sym;
   self.context[d[0] as usize][d[1] as usize] = sym;
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
      self.context[mover[0] as usize][mover[1] as usize] = sym;
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
      self.context[mover[0] as usize][mover[1] as usize] = sym;
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
       self.context[mover[0] as usize][mover[1] as usize] = sym;
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
       self.context[mover[0] as usize][mover[1] as usize] = sym;
   };
}
}
pub struct Field<'a>{
   pub seq: Vec<&'a mut Layer<'a>>,
   pub default_char: char,
}

impl Field<'_>{
   pub fn unite<'a>(&'a self, layers:&'a Vec<&mut Layer>) -> Vec<Vec<&Character>>{
      let mut layer_result: Vec<Vec<&Character>> = layers[0].copy([0, 0], [layers[0].x, layers[0].y]);
      //println!("{:?}", layer_result);
      for layer in 1..layers.len() {
         for x in 0..layers[layer].context.len() { 
            for tile in 0..layers[layer].context[x].len() {
               if layers[layer].context[x][tile].context != self.default_char {
                  layer_result[x][tile] = layers[layer].context[x][tile];
               };
               //println!("l: {}, x: {}, y: {}, c: {}, is space: {}", x, y, tile, layers[x][y][tile], layers[x][y][tile] == self.default_char);
            };
         };
      };
      layer_result
   }
   pub fn print(&self){
      let all_layers = self.unite(&self.seq);
      for y in 0..all_layers.len()-1 {
         println!("");
         for x in 0..all_layers[0].len()-1 {
            print!("{}", all_layers[x][y].context.to_string().truecolor(all_layers[x][y].red, all_layers[x][y].green, all_layers[x][y].blue));
         };
      };
   }
   pub fn print_sepchar(&self, sepchar:char){
      let all_layers = self.unite(&self.seq);

      for y in 0..all_layers.len() - 1 {
         println!("");
         for x in 0..all_layers[0].len() - 1 {
            print!("{}{}", all_layers[x][y].context, sepchar);
         };
      };
   }
   
}

fn main(){
   let aa = Character{context: '<', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ab = Character{context: '(', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ac = Character{context: '°', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ad = Character{context: ' ', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ae = Character{context: ')', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ag = Character{context: '/', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ah = Character{context: '=', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let ai = Character{context: '|', red: 0, green: 255, blue: 0, unicode: "".to_string()};
   let land_char: Character = Character {context: '*', red: 192, green: 128, blue: 32, unicode: "".to_string()};
   let air_char: Character = Character {context: ' ', red: 0, green: 0, blue: 0, unicode: "".to_string()};
   let dino: Vec<Vec<&Character>> = vec![
      vec![
         &aa,
         &ab,
         &ac,
         &ad,
         &ae
      ],
      vec![
         &aa,
         &ag,
         &ad,
         &ah,
         &ad
      ],
      vec![
         &ag,
         &ai,
         &ai,
         &ad,
         &ad
      ],
   ];
   
   let mut field: Field = Field {seq: Vec::new(), default_char: ' '};
   let mut main_layer: Layer = Layer {x: 32, y: 16, context: Vec::new()};
   main_layer.build(&air_char);   
   main_layer.horizontal([0, 32], 15, &land_char);
   
   let mut dino_layer: Layer = Layer {x: 32, y: 16, context: Vec::new()};
   dino_layer.build(&air_char);
   dino_layer.paste(&dino, [0, 12]);
   
   field.seq.push(&mut main_layer);
   field.seq.push(&mut dino_layer);
   field.print()
}
