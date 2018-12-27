#[derive(Debug)]
struct User{
  username: String,
  email: String,
  sign_in_count:u64,
  active:bool,
}

struct Colors(u32,u32,u32);

fn build_User(email: String, username: String) -> User{

    User{
      email:email,
      username: username,
      active : true,
      sign_in_count :2,
    }



}

struct Rectangle{
  width:u32,
  height:u32,
}

impl Rectangle{
    fn area(&self)-> u32{ //use &self to prevent creating a copy and taking ownership. This allows us to burrow self and read it.
      self.width * self.height
    }

    fn can_hold(&self, rec1: &Rectangle)-> bool{
        if self.width > rec1.width && self.height > rec1.height {
          return true;
        }else{
          return false;
        }

    }

    fn square(size: u32) -> Rectangle { //associative function its called by using scope resolution with Rectangle class
        Rectangle{width:size, height: size}
    }
}



fn main() {
    println!("Hello, world!");
    let mut user1 = User{ // Whole instance must be mutable. No such thing as partial mutable instances/ structs
    email: String::from("Hello@gmail.com"),
    username: String::from("RONNY"),
    active: true,
    sign_in_count: 1,
    };

    user1.email = String::from("Newemail@gmail.com");
    let user2 = build_User(String::from("Ronny"),String::from("Ronny"));

    println!("{}",user2.email); // must remember to use the placement specifier "{}" to place value in statement.


    let user3 = User {
    ..user1
    }; // update syntax example: replicates the values of an instance without specifying the remaining members of that instance. (Copy constructor similarities);
    println!("{}",user3.email);

    let color1 = Colors(255,255,255);
    println! ("{}",color1.0); // tuple structs are access in the same fashion as arrays and tuples with the . and then index of element.

    let rect1 = Rectangle {width: 30, height : 50};
    let rect2 = Rectangle {width: 25, height : 49};

    let rect3:Rectangle = Rectangle::square(30);

    println!("{}",rect2.area());
}
