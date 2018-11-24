use std::io;

fn main() {
    println!("Whats the index of the fib number you want?(index starts at 0)");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read line");

    let index : usize = index.trim().parse().expect("Must be a positive number!");

    println!("fib at index {} equals {}",index,fibs(index));
}

fn fibs(x: usize)-> usize{ //this function will overflow at 94 for 64bit integers
                           // and at 48 for 32bit integers
  let mut f1:usize = 0;
  let mut f2:usize = 1;

  if x==1 {
    return f2;
  }else if x==0 {
    return f1;
  }

  for _i in 1..x{
      let tempf3 :usize = f2+f1;
      f1=f2;
      f2=tempf3;
  }

  return f2;

}
