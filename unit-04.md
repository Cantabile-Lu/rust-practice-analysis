## [基本类型](https://zh.practice.rs/basic-types/intro.html)

1. 整数🌟

这题需要注意的是在Rust中：

- 如果声明的类型不同不能赋值

可以显示绑定相同类型， 或者编译器自动帮我们推导一个类型。

正确代码如下

```rust
fn main() {
    let x: i32 = 5;
  - let mut y: u32 = 5;
  + let mut y = 5;
    y = x;
    println!("{y}");   
    let _z = 10; // 未使用的变量则需要添加_ 
}
```

2. 🌟

- 不同类型不能赋值`as`关键字可以强制转类型转换

正确代码如下：

```rust
fn main() {
    - let v: u16 = 38_u8 as __;
    + let v: u16 = 38_u8 as u16;
    println!("{v}")
}
```

3. 🌟🌟🌟

这题需要注意的是在Rust中：

- 整形默认使用的是`i32`

正确代码如下：

```rust
fn main() {
    let x = 5;// 默认为 i32
    - assert_eq!("u32".to_string(), type_of(&x));
    + assert_eq!("i32".to_string(), type_of(&x));
}
// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```

4. 🌟🌟

这题需要注意的是在Rust中：

- `i8`的数字范围值是`-(2^n - 1) ~ (2^n - 1) - 1` 即`-(2^7) ~ 2^7 - 1` = -128 ~ 127
- `u8`的数字范围是`0 ~ 2^n - 1` 即 `0 ~ 2^8 - 1`  = 0 ~ 255
- `::MAX`表示范围内最大值。

正确代码如下：

```rust
fn main() {
   - assert_eq!(i8::MAX, __); 
   - assert_eq!(u8::MAX, __); 
   + assert_eq!(i8::MAX, 127); 
   + assert_eq!(u8::MAX, 255); 
}
```

5. 🌟🌟

这题需要注意的是在Rust中：

- 已知`u8`的最大存储值是`255`, 那么 `251 + 8` 结果是`259`显然已经超过`u8`类型的取值范围，这个时候就会溢出， 编译器也会显示错误。
- `::checked_add`执行加法并检查溢出， 如果没有溢出返回加法结果，反之会导致`panic`

正确代码如下：

```rust
fn main() {
    -let v1 = 251_u8 + 8;
    +let v1 = 100_u8 + 8;
    -let v2 = i8::checked_add(251, 8).unwrap();
    +let v2 = i8::checked_add(100, 8).unwrap();
    println!("{},{}",v1,v2);
 }
```

6. 🌟🌟

这题需要注意的是在Rust中：

- `assert`是一个断言函数， 断言是否为真。
- `1_024`表示十进制1024
- `0xff`表示十六进制数255
- `0o77`表示八进制数63
- `0b1111_1111`表示二进制数数255

将结果相加断言是否和1579相等

正确代码如下：

```rust
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    +println!("{v}"); //获得相加结果
    -assert!(v == 1579);
    +assert!(v ！= 1579);
}
```

7. 🌟

这题需要注意的是在Rust中：

- 默认浮点类型是`f64`

```rust
fn main() {
    let x = 1_000.000_1; // 所以浮点类型是f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}
```



