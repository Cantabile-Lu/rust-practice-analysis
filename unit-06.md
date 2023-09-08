### [复合类型-字符串](https://zh.practice.rs/compound-types/string.html)

1. 🌟

需要注意的是：

- 正常情况下我们无法使用 `str` 类型，但是可以使用 `&str`

正确代码如下：

```rust
fn main() {
-    let s: str = "hello, world";
+    let s: &str = "hello, world";
}
```

2. 🌟🌟

需要注意的是：

正确代码如下：

```rust
// 1
fn main() {
    let s: Box<str> = "hello, world".into();
-   greetings(s)
+   greetings(&s)
}
fn greetings(s: &str) {
    println!("{}",s)
}
// 2: 
fn main() {
-   let s: Box<str> = "hello, world".into();
+   let s:&str = "hello, world";
    greetings(s)
}
fn greetings(s: &str) {
    println!("{}",s)
}
```

3. 🌟

需要注意的是：Rust中创建字符串的方法很多，这里列举其中几个

- `String::new()`创建一个新的空 `String`，初始化不会分配任何缓冲区，添加数据时可能会导致过多的分配。
- `String::with_capacity(number)`创建一个至少具有指定容量的新空 `String`。
- `String::from("")`创建一个具有初始值的`String`。
- `"".to_string()`一个用于将值转换为 `String` 的 trait。

```rust
let mut s = String::new();
let mut s = String::with_capacity(10);
let mut s = String::from("");
let mut s = "".to_string();
```

正确代码如下：

```rust
fn main() {
-   let mut s = __;
+   let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    // println!("{s}");
    assert_eq!(s, "hello, world!");
}
```

4. 🌟🌟🌟

需要注意的是：

- `String`类型的`push`将给定的 [`char`](https://rustwiki.org/zh-CN/std/primitive.char.html) 追加到该 `String` 的末尾。
- `String`类型的`push_str`将给定的字符切片即`&str`追加到末尾。
- `String`类型的`+=`将给定的字符串切片即`&str`追加到末尾。
- 这些方法都是在原有的字符串追加，并不会返回新字符串，所以该字符串必须是可变的。

正确代码如下：

```rust
fn main() {
-    let  s = String::from("hello");
-    s.push(',');
-    s.push(" world");
-    s += "!".to_string();
    
+    let mut  s = String::from("hello");
+    s.push(',');
+    s.push_str(" world");
+	 s += "!";
    println!("{}", s)
}
```

