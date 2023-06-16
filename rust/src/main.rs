#[derive(Debug)]
struct Field{
   matrix: Vec<Vec<char>>,
   x: usize,
   y: usize,
}
impl Field{
   fn begin(&mut self, x:usize, y:usize){
      for step_x in 0..x {
         let empty_vec: Vec<char> = Vec::new();
         self.matrix.push(empty_vec);
         for step_y in 0..y {
            self.matrix[step_x].push('#');
         };
      };
      println!("{:#?}", &self.matrix);
   }
}
fn main() {
   const x: usize = 10;
   const y: usize = 10;
   let mut field = Field{
      matrix: Vec::new(),
      x: x,
      y: y,
   };
   field.begin(x, y);
}