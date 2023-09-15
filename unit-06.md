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

- `into`是一个常用的转换方法，它将一个类型转换成另一个类型。
- 函数形参标注的类型就决定了函数实参的类型。

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
    let s: Box<str> = "hello, world".into();
    greetings(s)
}
-fn greetings(s: &str) {
-    println!("{}",s)
-}
+fn greetings(s: Box<str>) {
+    println!("{}",s)
+}
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

5. 🌟🌟

正确代码如下：

```rust
fn main() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
-    let s1 = s._("dogs", "cats");
+    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats")
}
```

6. 🌟🌟

需要注意的是： 为什么`s2`不能与`s1`调换位置？

正确代码如下：

```rust
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
-    let s3 = s1 + s2; 
+    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}
```

7. 🌟🌟

正确代码如下：

```rust
// 1
fn main() {
-    let s = "hello, world";
+    let s = "hello, world".into();
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}
// 2
fn main() {
    let s = "hello, world";
    greetings(s)
}

-fn greetings(s: String) {
-    println!("{}",s)
-}

+fn greetings(s: &str) {
+    println!("{}",s)
+}
```

8. 🌟🌟

正确代码如下：

```rust
fn main() {
    // 1: 
    let s = "hello, world".to_string();
    let s1: &str = &s;

    // 2: 
    let s = "hello, world";
    let s1: &str = s;
}
```

9. 🌟

正确代码如下：

```rust
fn main() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );
    // // 还能使用 \ 来连接多行字符串
    let long_string = "String literals\
                        can span multiple lines.\
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
```

10. 🌟🌟🌟

需要注意的是：

- `r`是原始字符串，使用其目的是为了保持字符串的原义， 不需要转义。
- 使用原始字符串中需要包含`""`则需要在两侧添加`#`，如字符串中包含`#`则需要更多的`#`。

正确代码如下

```rust
fn main() {
-   let raw_str = r"Escapes don't work here: \x3F \u{211D}";
+   let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    
-   let long_delimiter = __;
+   let long_delimiter = "Hello, \"##\"";
+   let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}
```

11. 🌟🌟 

需要注意的是：

- 在`utf-8`的编码模式下英文占1字节， 中文占用3字节。
- 切片的索引必须落在字符之间的边界位置。

```rust
    // 汉字
    let s  = '中'; // 3
    // 特殊符号
    let s = '⊙'; // 3
    // 图标
    let s  = '❗'; // 3
    // 复杂图标
    let s = '🚒'; // 4
    // 英文
    let s  = 'a'; // 1
    // 标点符号
    let s  = '!'; // 1
    let byte = s.len_utf8();
    println!("{:?}", byte)
```

正确代码如下：

```rust
fn main() {
    let s1 = String::from("hi,中国");
-    let h = s1[0];
+    let h = &s1[0..1]; 
    assert_eq!(h, "h");

-    let h1 = &s1[3..5];
+   let h1 = &s1[3..6];
    assert_eq!(h1, "中");
}
```

12. 🌟

需要注意的是：

- 在Rust中能被`for`循环的一定是实现了迭代器（Iterator）相关方法的。
- `chars`方法能返会一个实现了`Iterator<Item = char>`特性的迭代器。

正确代码如下：

```rust
fn main() {
-    for c in "你好，世界".__ {
-        println!("{}", c)
-    }
    
+    for c in "你好，世界".chars() {
+        println!("{}", c)
+    }
}
```

