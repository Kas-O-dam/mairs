# Documentation
Hey! This docs haven't so official speach, because I am bad in english and can not describe my lib else then so.
Mairs has one good tool - layers. The layer is struct with properties: x, y, context. So in fact layer is canva and all matrix you must keep in property "context" with datatype Vec<Vec<&Character>>, about Character we shall talking later.
```Rust
let mut layer = Layer {
	x: 10,
	y: 10,
	context: Vec::new() // Vec<Vec<&Character>>
};
layer.build();
```
But if you create the layer example, then you can not print that into terminal. You must create one struct for programme: it is struct "Field". if the layers need as character canva, field need for do manipulation between layers, so for use this feature, push your layer to property "seq" (sequence) in Field example. You must put empty vector in seq and character for opacity, I prefer specify space
```Rust
let mut field = Field {
	seq: Vec::new(), // vector with layers
	default_char: ' ', // char as opacity
};
field.seq.push(&layer);
```
Yeeeees! We use links in this version!
Then you can print your layer(s) into terminal, the method "print" shows all layers in property seq to terminal from first to last and does not show, when see a character equal default_char.
Field has another one method - print_sepchar (print and separate characters), where you can create some space between characters as character. So this call:
```Rust
field.print_sepchar('!');
```
Will show like this:
```
  @           ! !@! ! !
 @@@    =>    !@!@!@! !
@@@@@        @!@!@!@!@!
```
And the lastest of methods of Field is unite. It, how you already understand, unite layers to one. specify vector of layers to arguments and get one <u>Vec<Vec<&Character>></u>, not layer. 
The all of hard is behind, then I made a list with short description for Layer's methods
- line: draw line. You must specify coordinates of begin as array of unsigned size and end to this type too, then character for filling
- horizontal: draw line with constable y. You must specify x in first as array (with length 2) of unsigned size and y as usize in second, then character for filling
- vertical: similar but in first you specify one unsigned size for x and array (with length 2) for y.
- round: draw round without filling. In first coordinates (array with length 2) of center in <u>i32</u> and radius also in i32 and character
- rect: draw rect without filling. In first coordinates (array with length 2) as usize of left-top corner, then such for right-bottom corner and character
- cut: return region as Vec<Vec<&Character>>, need to coordinates for begin and end (left-top & right-bottom corners) (array with length 2) in usize type and character, which will be put to cut characters
- copy: like cut, but save characters in them places and doesn't need to character argument
- paste: place some slice (Vec<Vec<&Character>>) into our Layer. specify slice's link in first and coordinates of left-top corner in second (array with length 2 as usize)

spoiler: I want add "overlapping" method

And last object of our travel is Character struct. It keep a chartype in property "context", and! O my God! I DID CODED COLOURED OUTPUT! specify properties red, green and blue for colourize. Also the struct has a unicode property. Why I add that? idk. Put here something

End. now, you know all