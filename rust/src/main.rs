#[derive(Debug)]

struct Layer{
   matrix: Vec<Vec<char>>,
   x: usize,
   y: usize,
}
impl Layer{
   fn build_matrix(&mut self, x:usize, y:usize) -> Vec<Vec<char>>{
      let mut layer: Vec<Vec<char>> = Vec::new();
      for _step_x in 0..x {
         let empty_vec: Vec<char> = Vec::new();
         layer.push(empty_vec);
         for _step_y in 0..y {
            layer[_step_x].push('#');
         };
      };
      layer
   }
   fn print(&self){
      for y in 0..self.y {
         println!("");
         for x in 0..self.x {
            print!("{}", self.matrix[x][y]);
         };
      };
   }
   /*
   	 _        _    _    _    ______
		| |      | |  | \_ | |  |  ____|
		| |      | |  | |\\| |  | |----,
		| |____  | |  | | \\ |  | |----'
		|_____|  |_|  |_|  \_|  |______|
   */
   fn line(&mut self, begin:[i32; 2], end:[i32; 2], sym:char) -> Vec<Vec<char>>{
      let mut layer: Vec<Vec<char>> = self.build_matrix(self.x, self.y);
      if (begin[0] - end[0]).abs() >= (begin[1]-end[1]).abs(){
         println!("build by x");
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
            println!("x: {}  y: {}  e: {}  ck: {}", x, y, error, corner_koef);
            error += corner_koef;
         };
         for coors in &points{
            layer[coors[0]][coors[1]] = sym;
         };
      }else{
         println!("build by y");
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
            println!("x: {}  y: {}  e: {}  ck: {}", x, y, error, corner_koef);
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
   let mut layer = Layer{
      matrix: Vec::new(),
      x: X,
      y: Y,
   };
   layer.build_matrix(X, Y);

   // "exams" for line function
   //layer.matrix = layer.line([5, 4], [9, 4], '0'); // a
   //layer.matrix = layer.line([9, 4], [5, 4], '1'); // a reverse
   //layer.matrix = layer.line([5, 4], [9, 2], '0'); // b
   //layer.matrix = layer.line([9, 2], [5, 4], '1'); // b reverse
   //layer.matrix = layer.line([5, 4], [9, 0], '0'); // c
   //layer.matrix = layer.line([9, 0], [5, 4], '1'); // c reverse
   //layer.matrix = layer.line([5, 4], [7, 0], '0'); // d
   //layer.matrix = layer.line([7, 0], [5, 4], '1'); // d reverse
   //layer.matrix = layer.line([5, 4], [5, 0], '0'); // e
   //layer.matrix = layer.line([5, 0], [5, 4], '1'); // e reverse
   //layer.print();
}