/* 1 

fn main(){
    // 默认格式化
    let a = "Cantabile";
    println!("{}", a); // Cantabile
    // 包含类型信息 
    let a = "Cantabile";
    println!("{:?}", a); // "Cantabile"
    // 使用科学计数法特性格式化
    let a = 12_i32;
    println!("{:e}", a); // 1.2e1
    // 使用二进制格式化
    let a = 12_i32;
    println!("{:b}", a); // 1100
    //使用十六进制格式化
    let a = 42_i32;
    println!("{:x}", a); // 2a
    // 使用存储位置格式化
    let a = "Cantabile";
    println!("{:p}", a);//0x7ff69ef1f428
    
}
 fn main() {
    let x = &4;
    // 填写空白处
    println!("x 的内存地址是 {:p}", x); // output: 0x7ff78d38e428
 }

fn main() {
    let x = 5;
    // 填写空白处
    let p = &x;
    assert_eq!(5, *p);
 
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
 }*/

/* 2
 fn main() {
    let x = 5;
    let y = &x;
    // 只能修改以下行
    assert_eq!(5, *y);
} */

/* 3. 
fn main() {
    let  s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}

*/
/* 4

 fn main() {
    let mut s = String::from("hello, ");
    push_str(&mut s)
}
fn push_str(s: &mut String) {
    s.push_str("world")
}
*/

/* 5. 

fn main() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let  p = &mut s;
    
    p.push_str("world");
}
*/

/* 6 
fn main() {
    let ref value = Some(String::from("Cantabile"));

    match value {
        Some(x) => println!(" {}", x),
        _ => println!("No value"),
    }

    println!("{:?}",value);
}


fn main() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
*/


/* 7
fn main() {
    let mut s = String::from("Cantabile");
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
}
 */
/* 8. 

fn main() {
    // 通过修改下面一行代码来修复错误
    let   s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {
    println!("{s}")
}

*/
/* 9 

// 下面的代码没有任何错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    s.push_str("world");
}

fn borrow_object(s: &String) {
    println!("{}",s.len())
}
*/

/* 10.
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}",r2);
}
 */

 /* 11 */
fn main() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once a...
    println!("{}",r1);
    // 你不能同时使用 r1 和 r2
}
