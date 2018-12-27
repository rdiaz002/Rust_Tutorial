enum IpAddrKind{
  V4(String),
  V6(String),
}

/*struct IpAddr{
  kind: IpAddrKind,
  address: String,
}

let hom = IpAddr{
kind:IpAddrKind::v4,
address: String::from("127.0.0.1"),
};

let loopback = IpAddr{
    kind: IpAddrKind::v6,
    address: String::from("::1"),
};*/ //negate this by associating enum variables to values.



enum Message{
  Quit,
  Move{ x:i32, y:i32},
  Write(String),
  ChangeColor(i32,i32,i32),
}

impl Message{
  fn call(&self){

  }
}

fn main() {
    let six = IpAddrKind::V6(String::from("::1"));
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    //Option
    let some_numer = Some(5);
    let some_string = Some("a string");

    let absent_numer:Option<i32> = None;




}
