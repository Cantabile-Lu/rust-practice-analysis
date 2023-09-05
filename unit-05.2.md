### [引用与借用](https://zh.practice.rs/ownership/borrowing.html)

1. 🌟

需要注意的是：

- 常规的引用是一个指针类型，指向了对象存储的内存地址。
- `println!`用于在终端或控制台上打印文本，它是`std::fmt`其中之一的宏，在使用`println!`时可以使用特定的参数进行格式化输出。

```rust
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
```

正确代码如下：

```rust
fn main() {
    let x = 5;
    // 填写空白处
-   let p = __;
+   let p = &x; // 引用了变量x在栈上的内存地址
+   assert_eq!(5, *p);
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
 }
```

2. 🌟

需要注意的是：

- 引用过后，需要使用解引用运算符`*`来解出引用所指向的值。

```rust
fn main() {
    let x = 5;
    let y = &x;
    // 只能修改以下行
-   assert_eq!(5, y);
+   assert_eq!(5, *y);
}
```

3. 🌟

需要注意的是：

- 函数签名中在类型声明上，如果存在运算符`&`，那么当调用函数时，必须使用`&`运算符借用一个值。

正确代码如下

```rust
fn main() {
-   let mut s = String::from("hello, ");
+   let  s = String::from("hello, ");
-   borrow_object(s)
+ 	borrow_object(&mut s)
}
fn borrow_object(s: &String) {}
```

4. 🌟

需要注意的是：

- 创建可变引用需要使用`&mut`，并更新函数签名以接收一个可变引用，这也非常清楚的表明，函数将改变它所借用的值

```rust
let mut s = String::from("Cantabile");
do_something(&mut s)
fn do_something(s: &mut String){}  
```

正确代码如下

```rust
fn main() {
    let mut s = String::from("hello, ");
-   push_str(s)
+ 	push_str(&mut s)
}
fn push_str(s: &mut String) {
    s.push_str("world")
}
```

5.  🌟🌟

需要注意的是：

- 创建一个可变引用时，被引用的变量本身是需要可变的，可变引用不需要再次使用关键字`mut`进行标记。

正确代码如下：

```rust
fn main() {
    let mut s = String::from("hello, ");
-    let p = __;
+    let p = &mut s;
    p.push_str("world");
}
```

6. 🌟🌟🌟

需要注意的是：

- `ref`关键字创建一个引用绑定，其主要用于模式匹配`(match)`的上下文中。
- 使用`ref`与`&`都是创建引用，并将引用赋值给变量，它们的行为是相似的，都不会改变所有权，`&`用于一般的赋值和引用的创建。 

```rust
fn main() {
    let  value = Some(String::from("Cantabile"));
    match value {
        Some(n) => println!(" {}", x),
        _ => println!("No value"),
    }
    println!("{:?}",value);//Error:  value borrowed here after partial move
}

fn main() {
    let  value = Some(String::from("Cantabile"));
    match value {
        Some(ref n) => println!(" {}", x),
        _ => println!("No value"),
    }
    println!("{:?}",value); // Some("Cantabile")
}

```

正确代码如下：

```rust
fn main() {
    let c = '中';
    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
-   let __ r2 = c; 
+   let ref r2 = c;
    assert_eq!(*r1, *r2);   
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
}
// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```

7. 🌟

需要注意的是：

- Rust的借用规则确保在任何给定时间只有一个可变引用活跃在用一时间读取和写入数据，以避免数据竞争。

```Rust
    let mut s = String::from("Cantabile");
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
```

如上代码中可变引用`r1`在后续没有继续使用，所以引用在`println`之后进行了释放， `r1` 和 `r2` 不会同时活跃，因此没有违反 Rust 的可变引用规则

Rust也允许多个可变引用同时共存，只要它们没有同时进行写入操作。

```rust
    let mut s = String::from("Cantabile");
    let r1 = &mut s;
    let r2 = &mut s;
```

正确代码如下：

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
-   println!("{},{}",r1, r2);// 
+   println!("{},", r2);
}
```

8. 🌟 错误: 从不可变对象借用可变

需要注意的是：

- 创建一个可变引用时，被引用的变量本身是需要可变的

正确代码如下：

```rust
fn main() {
-   let  s = String::from("hello, "); 
+   let mut  s = String::from("hello, ");
    borrow_object(&mut s)
}
fn borrow_object(s: &mut String) {}
```

9. 🌟🌟

- 从可变对象借用不可变是可以的， 需要注意的是，标明了不可变就不能对值有更改操作。

```rust
fn main() {
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
}

fn borrow_object(s: &String) {
+ 	 // s.push_str("world"); // Error
+    println!("{}",s.len()) // success
}
```

10. 🌟🌟

需要注意的是：

- 对可变引用的释放是由变量离开作用域来触发的。

正确代码如下：

```rust
fn main() {
    let mut s = String::from("hello, ");
    let r1 = &mut s; // r1 的作用域开始
    r1.push_str("world");
    // r1 的作用域结束，r1 的可变引用被释放
    let r2 = &mut s; // r2 的作用域开始
    r2.push_str("!");
    // r2 的作用域结束，r2 的可变引用被释放
-   println!("{}",r1);
+   //println!("{}",r2);
}
```

在如上代码中`r1`的变量作用域在`r1.push_str`之后结束，`r1`的可变引用被释放，所以可以创建`r2`的可变引用。

11. 🌟🌟

需要注意的是：

- 不能在可变引用创建和使用之间创建新的同一数据可变引用。

正确代码如下：

```rust
fn main() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
+   println!("{}",r1);
}
```

