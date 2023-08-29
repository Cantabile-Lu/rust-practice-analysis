/* 
    数值类型 
*/
/* 1: 🌟 移除某个部分让代码工作 
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    println!("{y}", y = y);
    
    let _z = 10; // 这里 z 的类型是? 
}
*/

/* 2: 🌟
fn main() {
    let v: u16 = 38_u8 as u16;
    println!("{v}")
}
 */


/*  3: 🌟🌟🌟 

// 修改 `assert_eq!` 让代码工作
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/

/* 4:  🌟🌟
// 这里就是取 i8 / u8 能取得最大值
// i8 : -2(^n) ~ 2^n - 1
// u8: 0 ~ 2^n - 1
fn main() {
    assert_eq!(i8::MAX, 127 );  
    assert_eq!(u8::MAX, 255); 
}
*/
/* 5:  🌟🌟
fn main() {
    let v1 = 100_u8 + 8;
    let v2 = i8::checked_add(100, 8).unwrap();
    println!("{},{}",v1,v2);
 }*/

/* 6:  🌟🌟
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{v}");
    assert!(v != 1579);
}
*/


/* 7:  🌟🌟 
fn main() {
    let x = 1_000.000_1; // ? 1000.0001
    println!("{x}");
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}
*/


/* 8:  */
fn main() {
    assert!(0.1_f32+0.2_f32==0.3);
}
