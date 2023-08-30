## [基本类型](https://zh.practice.rs/basic-types/intro.html)

###### [数值类型](https://zh.practice.rs/basic-types/numbers.html)

需要注意的是在Rust中：

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

需要注意的是在Rust中：

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

需要注意的是在Rust中：

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

需要注意的是在Rust中：

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

需要注意的是在Rust中：

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

需要注意的是在Rust中：

- 默认浮点类型是`f64`

```rust
fn main() {
    let x = 1_000.000_1; // 所以浮点类型是f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}
```

8. 🌟🌟 使用两种方法来让下面代码工作

需要注意的是在Rust中：

- 默认类型是`f64`，其精度比`f32`更高

因为二进制的精度问题，所以导致了进制转换带来的偏差，如果要解决这个问题，可以使用`f32`类型

```rust
let v = 0.1_f64+ 0.2_f64;
println!("{v}"); // 0.30000000000000004

let v = 0.1_f32+ 0.2_f32;
println!("{v}"); // 0.3
```

正确代码如下： 

```rust
fn main() {
    let v = 0.1_f64+ 0.2_f64;
    println!("{v}"); // 0.30000000000000004

    let v = 0.1_f32+ 0.2_f32;
    println!("{v}"); // 0.3

   let v = (0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 0.000001;
    println!("{v}"); // true
}
```

9. 🌟🌟 两个目标: 1. 修改 `assert!` 让它工作 2. 让 `println!` 输出: 97 - 122

需要注意的是：

- `..`与`..=`的区别是前者创建的范围从开始值到结束值，且不包含结束值，后者开始值和结束值都在内。

所以`-3..2`起始是`-3`结束是`1`， 进行运算后的结果就是 `-2`

```rust
for i in -3..2 {
    println!("{i}"); // -3  -2 -1 0 1
}
```

- `'a'..='z'`生成一个字符范围，包括`a`和`z`, 在迭代字符时，实际上是在迭代Unicode编码值的范围，其字符`a`对应的编码值正是97`b`对应的编码值是98以此类推。

所以我们可以通过`as`关键字强制转换类型为数值类型。

正确代码如下：

```rust
fn main() {
    let mut sum = 0;
    -for i in -3..2 {
    -  sum += i
    -}
    
    +for i in -3..=2 {
    +   sum += i
    +}
   assert!(sum == -3);
    
    for c in 'a'..='z' {
       - println!("{}",c);
       + println!("{}", c as i8 );
    }
    // 将 数值类型转为a-z
   + for c in 97..=122 {
   +    let m = char::from(c);
   +    println!("{}", m);
   +}
    
}
```

10. 🌟🌟

需要注意的是：

- `std::ops`是Rust标准库中的模块，提供了许多操作符和运算符相关的特性和类型。
- `std::ops::Range`创建一个从起始值开始，不包含结束值的范围，其定义为`start <= x < end`
- `std::ops::RangeInclusive::new`创建一个包括起始值和结束值的范围，其定义为包含`x >= start`和`x <= end`的所有值。

是不是觉得：我们好像在哪儿见过，你记得吗？

是的， 这和`.. & ..=`同样是创建范围的表示方法 ，区别在于前者是结构体类型，后者是一个操作符。

```rust
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
```

正确代码如下：

```rust
use std::ops::{Range, RangeInclusive};
fn main() {
   - assert_eq!((1..__), Range{ start: 1, end: 5 });
   + assert_eq!((1..5), Range{ start: 1, end: 5 });
   - assert_eq!((1..__), RangeInclusive::new(1, 5));
   + assert_eq!((1..=5), RangeInclusive::new(1, 5));
}
```

11. 🌟

需要注意的是：

- 数值类型运算需要考虑到值是否在规定的范围内。

```rust
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
```

###### [字符、布尔、单元类型](https://zh.practice.rs/basic-types/char-bool-unit.html)

1. 🌟

需要注意的是`std::mem`是Rust标准库中的一个模块， 该模块包含用于查询类型的大小和对齐，初始化和操作内存函数。

- `size_of_val`的作用是返回指向值的大小（以字节为单位）。
- `&`是引用操作符，引用指向某个值的指针，它允许你使用值但不获取所有权。

- 由于`Unicode`都是4个字节编码，因此字符类型也是占用4个字节。

正确代码如下：

```rust
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
   - assert_eq!(size_of_val(&c1),1); 
   + assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
   - assert_eq!(size_of_val(&c2),3); 
   + assert_eq!(size_of_val(&c2),4); 
    println!("Success!")
} 
```

2.  🌟

需要注意的是：

- 在Rust中`''` （单引号）与`""`（双引号）并不相同，字符类型使用的是单引号， 而双引号是字符串

正确代码如下： 

```rust
fn main() {
   - let c1 = "中";
   + let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
```

3. 🌟

需要注意的是：

- `!`运算符在Rust中有多种用法，`!`是什么用法取决于它的前后文。

比如宏调用

```rust
println!("{}");
```

比如： 逻辑非

```rust
let is_true = true;
if !isTrue { ..};
```

比如： 在函数从不返回的时候充当返回值，值得注意的是，这样的写法通常是通过插入一些无限循环或在结束时引起`panic`。

```rust
fn bar() -> ! {panic!("This is a panic!");}
```

正确代码如下：

```rust
fn main() {
    let _f: bool = false;

    let t = true;
    - if !t {
    + if !!t { || if t {
        println!("Success!")
    }
} 
```

4. 🌟

需要注意的是：`&&`逻辑与运算符的规则如下：

- 如果左侧操作数为`false`，那么整个表达式的结果就是`false`，不计算右侧
- 如果左侧的操作数为`true`， 那么整个表达式的结果取决于右侧操作数

```rust
true && false // false 
false && true  // false
true && true // true
```

正确代码如下：

```rust
fn main() {
    - let f = true;
    + let f = false;
    let t = true && false;
    assert_eq!(t, f);
    println!("Success!")
}
```

5. 🌟🌟