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

fn main() {
    let value = Some(42);

    match value {
        Some(ref x) => println!("Got a reference to {}", x),
        None => println!("No value"),
    }
}
