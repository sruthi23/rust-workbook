fn main() {
      let width = 10;
      let height = 12;
let result = rectangle_area(width,height);
    println!("The area of rectangle is {}",result);
}

fn rectangle_area(i:u32, j:u32)->u32{
        return i*j;
}
