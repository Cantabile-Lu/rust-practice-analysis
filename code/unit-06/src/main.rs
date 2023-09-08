/* 1 
fn main() {
    let s: &str = "hello, world";
}
*/

/* 2. 
// 1 
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

2: 
fn main() {
    let s:&str = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
*/

/* 3: */
fn main() {
    let mut s = String::with_capacity(10);
    let mut s = String::from("");
    let mut s = "".to_string();
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    println!("{s}");
    assert_eq!(s, "hello, world!");
}



/* 4. 
fn main() {
    // let mut s = String::from("hello");
    // s.push_str(&','.to_string());
    // s.push_str(" world");
    // s += &"!".to_string();
    // println!("{}", s)

    // let mut s = String::from("hello");
    // s += &','.to_string();
    // s += &" world";
    // s += &"!".to_string();
    // println!("{}", s)

    let mut  s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}
*/
