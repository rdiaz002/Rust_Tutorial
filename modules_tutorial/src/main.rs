mod sound;

/*
    All items (functions, methods, structs, enums, modules, annd constants) are private by default.
    You can use the pub keyword to make an item public.
    You aren’t allowed to use private code defined in modules that are children of the current module.
    You are allowed to use any code defined in ancestor modules or the current module.
*/

mod movement{
    fn jump(){
        super::look_up();
    }
}

fn look_up(){

}

mod plants { // structs with public and private fields require a constructor
    pub struct Vegetable{
        pub name : String,
        id : i32,
    }
    impl Vegetable {
        pub fn new(name:&str) -> Vegetable{
            Vegetable{
                name : String::from(name),
                id:1,
            }
        }
    }
}

mod menu{ // enums are fully public when they are set to public
    pub enum Appetizer{
        soup,
        salad,
    }
}
use self::sound::instrument;
use std::{cmp::Ordering,io};//equivalent to : std::cmp::Ordering, std::io
//use std::io::{self,Write};// equivalent to std::io , std::io::Write

fn main() {
    crate::sound::instrument::clarinet();
    sound::instrument::clarinet();

    let mut v = plants::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious",v.name);

    instrument::clarinet();

}

/*Final notes:
When using and external package make sure you firstly, list them in your
packages cargo.toml, then bring them into scope using use keyword.
*/
