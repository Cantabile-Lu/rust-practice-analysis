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

- 在函数签名中`&类型`运算符表明接收一个`类型`的引用，那么当调用函数时，必须使用`&`运算符引用一个值。

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

- `ref`关键字



