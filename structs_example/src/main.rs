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


}
