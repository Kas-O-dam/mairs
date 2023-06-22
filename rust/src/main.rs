#[derive(Debug)]
struct Field{
   matrix: Vec<Vec<char>>,
   x: usize,
   y: usize,
}
impl Field{
   fn build_matrix(&mut self, x:usize, y:usize){
      for step_x in 0..x {
         let empty_vec: Vec<char> = Vec::new();
         self.matrix.push(empty_vec);
         for step_y in 0..y {
            self.matrix[step_x].push('#');
         };
      };
   }
   fn print(&self){
      for x in 0..self.x {
         println!("");
         for y in 0..self.y {
            print!("{}", self.matrix[x][y]);
         };
      };
   }
}
fn main(){
   const x: usize = 10;
   const y: usize = 10;
   let mut field = Field{
      matrix: Vec::new(),
      x: x,
      y: y,
   };
   field.build_matrix(x, y);
   field.print()
}