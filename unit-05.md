###### [所有权](https://zh.practice.rs/ownership/ownership.html)

1. 🌟🌟

需要注意的是：

- `String`是Rust标准库提供的内建类型，它是一个字符串对象，是一个长度可变得集合，使用`utf-8`作为底层数据编码格式，所以需要在堆上分配一块在编译时未知大小的内存存放内容。
- 因为`String`是存储在堆上的，所以不能通过赋值拷贝。

正确代码如下：

```rust
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
}
```

