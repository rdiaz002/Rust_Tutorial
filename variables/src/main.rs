fn main() {
const MAX_POINTS: u32 = 100_000;
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  let guess: u32 = "42".parse().expect("Not a Number1");

  let tuple: (u32,f32,i32) = (32,2.0,-2);

  let num = tuple.1;

  println!("The value of num is {}",num);

  let a = [1,2,3,4,5];

}
