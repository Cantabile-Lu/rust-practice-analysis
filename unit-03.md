## [变量绑定与解构](https://zh.practice.rs/variables.html)

1. 🌟 变量只有在初始化后才能被使用

需要得注意的是在Rust中：

-  允许申明未被初始化的变量， 但不允许使用未被赋值的变量。

-  Rust会对未使用的变量发出警示信息，如果想保留未使用的变量，则需要在变量名称前加`_`

正确代码如下：

```rust
fn main() {
    let x: i32 = 0; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}
```

2. 🌟🌟 可以使用 `mut` 将变量标记为可变   

需要注意的是在Rust中：

- 变量初始化后，默认不允许再修改该变量， 如果要修改变量的值，需要在变量前加上关键字`mut`表明这个变量是可变的。

正确代码如下：

```rust
fn main() {
    let mut x = 1;
    x += 2; 
    println!("x = {}", x); 
}
```

3. 🌟 作用域是一个变量在程序中能够保持合法的范围

需要注意的是在Rust中：

- 申明的变量只在当前作用域有效， 当变量离开当前作用域会被自动释放。

正确代码如下：

```rust
fn main() {
    let x: i32 = 10;
    let y: i32 = 10; //1 ： 新增代码。申明一个变量y, 并绑定值
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 
    //2 ： println!("x 的值是 {}", x);  不使用未申明的变量
}
```

4. 🌟🌟

需要注意的是在Rust中：

- 在`main`函数中使用了未声明的变量`x`
- 如果想使用函数`define_x`那么函数应该增加返回值，并标注返回值类型
- 其中`&'static str `是Rust中的一种类型表示。

如果这里没看懂也不用着急，把后面的知识学了再回头看这里，这里主要的还是理解变量的作用域为主。

```rust
fn main() {
    let x =  define_x();
    println!("{}, world", x); 
}
fn define_x() -> &'static str {
    let x = "hello";
    x
}
```

5. 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了

需要注意的是在Rust中：

- `assert_eq！`的作用是断言两个表达式相等。

根据作用域的规则，我们可以看到，当前作用域中的值是多少

```rust
    let x: i32 = 5; // 变量作用域 1
    {
    	// 变量作用域 2
        let x = 12;
        assert_eq!(x, 5);
    }
    assert_eq!(x, 12);
```

正确代码如下：

```rust
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
```

6. 🌟🌟 修改一行代码以通过编译

需要注意的是在Rust中：

- 如果要修改变量的值，需要在变量前加上关键字`mut`

正确代码如下：

```rust
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
```

7. 🌟 使用以下方法来修复编译器输出的 warning

需要注意的是在Rust中： 未使用的变量可以使用两种方式通过编译

- 在变量名称前增加`_`
- 使用`*#![allow(unused_variables)]*`

正确代码如下：

```rust
// 1: #![allow(unused_variables)]
#![allow(unused_variables)]
 fn main() {
    // 2: let _x = 1; 
    let x = 1; 
}
```

8. 🌟🌟 我们可以将 `let` 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量

需要注意的是在Rust中：

- 不管在任何情况下，只要是修改变量，就需要关键字`mut`，关键字的位置总是在变量前面

正确代码如下：

```rust
  fn main() {
    let (mut x, y) = (1, 2);
    x += 2; // 修改变量
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
```

9. 🌟🌟

需要注意的是在Rust中：

`..`运算符在解构赋值中用于表示忽略或省略的意思

关键部分是`(x,..) = (3,4) ; [.. , y] = [3, 4]`利用了Rust的解构赋值特性， 允许从一个数据解构中提取部分值并赋值给变量，同时忽略不需要的部分

正确代码如下：

```rust
fn main() {
    let (x, y);// 声明了 x 和 y 这两个变量，但未初始化
    (x,..) = (3, 4);// 利用解构赋值，将(3, 4) 的第一个值赋给 x，忽略了第二个值
    [.., y] = [1, 2];// 利用解构赋值，将 [1, 2] 的第二个值赋给 y，忽略了第一个值
    assert_eq!([x,y], [3,2]);
} 
```





