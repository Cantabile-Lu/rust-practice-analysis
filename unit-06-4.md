1. 🌟

需要注意的是：

- 元组的顺序是固定的，那么它的类型顺序也是固定的。 

正确代码如下：

```rust
fn main() {
    let _t0: (u8,i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
-   let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
+   let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}
```

2. 🌟 

需要注意的是：

- 元组成员的索引也是从0开始。

正确代码如下：

```rust
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
}
```

3. 🌟 

需要注意的是：

- 宏展开最大支持12个元素的元组，这是Rust编译器的限制，确保宏的性能和稳定性。

```rust
    let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    println!("{}", long_tuple.14)
```

正确代码如下：

```rust
fn main() {
-   let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,13);
+   let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
    
}
```

4. 🌟

需要注意的是：

- 解构元组变量的排列顺序就是元组值的排列顺序。

正确代码如下：

```rust
fn main() {
    let tup = (1, 6.4, "hello");
-    let __ = tup;
+    let (x,z,y) = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}
```

5. 🌟🌟

需要注意的是：

- 元组内解构的变量是一定要赋值的， 否则会引起编译错误。

```rust
let (x,y,z);
(x,y) = (1,2);// Error

let (x,y,z);
(x,y) = (1,2);
(z) = 3; // success
println!("{}, {}, {}", x,y, z)
```

正确代码如下：

```rust
fn main() {
    let (x, y, z);
    // 填空
-    __ = (1, 2, 3);
+   (y, z, x) = (1, 2, 3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
}
```

