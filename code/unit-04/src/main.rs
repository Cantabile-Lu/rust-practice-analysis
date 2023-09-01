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


/* 8:  
fn main() {
    let v = 0.1_f64+ 0.2_f64;
    println!("{v}"); // 0.30000000000000004

    let v = 0.1_f32+ 0.2_f32;
    println!("{v}"); // 0.3

    let v = (0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 0.000001;
    println!("{v}"); // true
}

*/

/* 9: 
fn main() {
    let mut sum = 0;
    for i in -3..=2 {
        sum += i
    }
    println!("{sum}");

    for c in 'a'..='z' {
        println!("{}", c as i8 );
    }

    
    // for c in 97..=122 {
    //     let m = char::from(c);
    //     println!("{}", m);
    // }
}
 */
/* 10  
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

// 测试代码
use std::ops::{Range, RangeInclusive};
fn main() {
    let range = Range{start: 3,end: 5};

    for i in range {
        println!("{}", i); // 输出 3, 4
    }

    for v in 3..5 {
        println!("{}", v); // 输出 3, 4
    }
    let range_1: RangeInclusive<i32> = RangeInclusive::new(3, 5);

    for i in range_1 {
        println!("{}", i); // 输出 3, 4, 5
    }

    for v1 in 3..=5 {
        println!("{}", v1); // 输出 3, 4, 5
    }
    
}
*/
/*  11 
  
fn main() {
    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); // u8类型的值是从0开始的, 所以需要更改类型
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);
    
    // // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
*/
/* 12 
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!")
} 
*/

/* 13 
fn main() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
*/
/* 14 
fn main() {
    let _f: bool = false;

    let t = true;
    if !!t {
        println!("Success!")
    }
} 


fn main() -> !{
     panic!("return !")
}
*/

/* 15 
fn main() {
    let f = true;
    let t = false && true;
    println!("{t}");
    assert_eq!(t, f);

    println!("Success!")
}
*/

/* 16
// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {

    assert_eq!((), implicitly_ret_unit());

    println!("Success!")
}


fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}


 fn do_something() -> () {
    println!("I will return a");
 }


use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
  */

  /* 17 
  fn main() {
   
    /* 1:   */
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
    assert_eq!(v,3);
    /* 2     */
  
    let v = {
        let x = 1;
        x + 2
    };
  
    assert_eq!(v, 3);

    let v = {
        let mut x = 1;
        x+= 2
    };
    assert_eq!(v, ());

 }
 */
/* 18 
 fn main() {
    let v =  3;
 
    assert!(v == 3);
 }
 */

 /*     19 
 fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, ());
    println!("Success")
}

fn sum(x: i32, y: i32) {
    x + y;
}
*/

/* 20
fn main() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x:i32, y: i32) -> i32 {
    x + y
}

// ******列子*********************************
 fn main() {
    let s = sum(1, 2);
    assert_eq!(s, ());
}

fn sum(x:i32, y: i32) -> () {
    x + y; // 语句默认返回（）既空的元组
}
 */

 /* 21 
 fn main() {
    print();
 }
 
 // 使用另一个类型来替代 i32
 fn print() {
    println!("hello,world");
 }
 */

 /* 22 
 fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    panic!("Stop");
    
}
*/

/* 23 
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
// 1.
fn never_return_fn() -> ! {
    panic!("This function will never return");
}

// 2:
use std::process::abort;
fn never_return_fn() -> ! {
    println!("aborting");
    abort()
}

// 3: 
use std::process::exit;
fn never_return_fn() -> ! {
    println!("aborting");
    exit(0x0100)
}

// 4:
fn never_return_fn() -> ! {
    loop{
        
    }
}
*/

/*  23 

fn main() {
    // 填空
    let b = true;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
*/


fn main(){
    let number = 12;
    match number {
        0 => {
            println!("number is 0")
        },
        1..=10 => {
            println!("number is between 1 an 10 ")
        },
        _n => {
            println!("number is {}", _n)
        }
    }
}
