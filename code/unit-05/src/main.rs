/* 1 
fn main() {
    // 1:
    let x = String::from("hello, Cantabile");
    let y = x.clone();
    println!("{},{}",x,y);

    // 2:
    let x = &String::from("hello, Cantabile");
    let y = x;
    println!("{},{}",x,y);

    // 3
    let x = String::from("hello, Cantabile");
    let y = x.to_string();
    println!("{},{}",x,y);

    // 4
    let x = String::from("hello, Cantabile");
    let y = &x[..];
    println!("{}, {}", x, y);

    // 5
    let x = String::from("hello, Cantabile");
    let y = &x;
    println!("{}, {}", x, y);
} */

/* 2: 

// 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
*/

/* 3.

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.clone().into_bytes();
    println!("{:?}",_s);// [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100]
    s
}
 */

 /* 4 
fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}
*/

/* 5 

// 不要使用 clone，使用 copy 的方式替代

fn main() {
    let x = (1, 2, (), "hello");
    let mut y = x;
    y.0 = 12;
    println!("{:?}, {:?}", x, y);

    let  x = (1, 2, (), "hello".to_string());
    let  y = & x;
    println!("{:?}, {:?}", x, y);
}

// fn main() {
//     let i:i32 = 5;
//     let five = String::from("5");
//     assert_eq!(five, i.to_string());
// }
*/

/* 6

fn main() {
    let  s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let mut  s1 =s.clone();

    s1.push_str("world")
}
 */

/* 7 

fn main() {
    let x = Box::new(5);
    
    let mut y = x.clone();
    *y = 4;
    assert_eq!(*x, 5);
}
// fn main(){
//     let mut a = Box::new(5);
//     *a = 2;
//     println!("{}", a)    
// }

*/

/* 8. 
fn main() {
    let a = (1,2);
    println!("{}",a.0);
    // 解构
    let (a,b)= (1,2);
    println!("{},{}",a,b);
    
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
 
    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);
 }
*/
 /* 9. */

fn main() {

    // let a = (2, true, false, 1.0);
    // let (s1,s2,s3,s4) = a;
    // println!("{:?},{:?},{:?},{:?},{:?}",s1,s2,s3,s4,a);

    // let t = (1, String::from("hello"),String::from("world"));
    
    // let (s1,s2,s3) = &t;
    // println!("{:?},{:?},{:?},{:?}",s1,s2,s3,t);

    let t = (String::from("hello"),String::from("world"));
 
    //填空，不要修改其它代码
    let (s1, s2) = &t;
 
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
