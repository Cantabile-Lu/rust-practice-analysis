/* 1 
fn main() {
    let s: &str = "hello, world";
}
*/

/* 2. 
// 1 
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

// 2: 
// fn main() {
//     let s:&str = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }
*/

/* 3: 
fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    // println!("{s}");
    assert_eq!(s, "hello, world!");
}
*/
