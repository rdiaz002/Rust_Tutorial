fn main() {
    println!("Hello, world!");
    let num = if another_function(10) > 5{
    10}
    else{
    3
    };
    println!("{}",num);

    let array = [10,20,50,40,20];

    for I in array.iter(){

      println!("The value is {}",I);

    }



}

fn another_function(x:i32)-> i32{

  if x > 5 {
  println!("The value of x is {}",x);
  }else {
    println!("This value sucks");
  }
  return x;
}
