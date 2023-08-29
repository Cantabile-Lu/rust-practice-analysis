/*
* 变量绑定与解构 
* 修复下面代码的错误并尽可能少的修改
*/

/* 1 🌟 变量只有在初始化后才能被使用
fn main() {
    let x: i32 = 0;
    let _y: i32;
    println!("x is equal to {}", x); 
}
*/

/* 2🌟🌟 可以使用 mut 将变量标记为可变*/
fn main() {
    let mut x = 1;
    x += 2; 
    println!("x = {}", x); 
}



/* 3🌟 作用域是一个变量在程序中能够保持合法的范围  
fn main() {
    let x: i32 = 10;
    let y: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 
}
*/

/* 4 🌟🌟
fn main() {
    let x = "hello";
    println!("{}, world", x); 
}

fn _define_x() {
    let _x = "hello";
}
 */

 /*  5 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing ) 
 fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}
*/

/* 
6 🌟🌟 修改一行代码以通过编译

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let mut x = x; 
    x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
    println!("{x},{y}")
}
 */

 /* 
 7: 使用以下方法来修复编译器输出的 warning :
🌟 一种方法
🌟🌟 两种方法

// 1: #![allow(unused_variables)]
#![allow(unused_variables)]
 fn main() {
    // 2: let _x = 1; 
    let x = 1; 
    
}
 */

 /* 
    8: 🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量

  fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
  */

  /* 
  9 : 🌟🌟

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3,2]);
} 
   */
