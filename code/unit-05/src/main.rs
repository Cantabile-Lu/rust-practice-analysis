/* 1  */
fn main() {
    // 1:
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);

    // 2:
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);

    // 3
    let x = String::from("hello, world");
    let y = x.to_string();
    println!("{},{}",x,y);

    // 4
    let x = String::from("hello, world");
    let y = &x[..];
    println!("{}, {}", x, y);

    // 5
    let x = String::from("hello, world");
    let y = &x;
    println!("{}, {}", x, y);
}

