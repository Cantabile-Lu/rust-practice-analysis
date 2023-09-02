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

2. 🌟🌟

需要注意的是：

- 如果一个函数有返回值，而且需要被变量保存并使用，那么这个函数是需要标明返回值类型。

正确代码如下：

```Rust
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

// 只能修改下面的代码!
-fn take_ownership(s: String) {
-    println!("{}", s);
-}

+fn take_ownership(s: String)  -> String {
+    println!("{}", s);
+    s
+}
```

3. 🌟🌟

需要注意的是：

- 在Rust一旦所有权被移动或者转移给另外一个变量，原始变量将不再有效，从而避免了内存泄漏和数据竞争等问题。
- 保留原始对象的所有权需要使用`clone`方法创建副本。

```rust
    let s1 = String::from("Cantabile");
    let s2 = s1.into_bytes();// s1 所有权已经移动
    println!("{}, world!", s1);// Error    -- move occurs because `s1` has...
```

正确代码如下：

```rust
fn main() {
    let s = give_ownership();
    println!("{}", s);
}
// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
   - let _s = s.into_bytes();
   + let _s = s.clone().into_bytes();
   + println!("{:?}",_s);// [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100]
   + s
}
```

4. 🌟🌟

需要注意的是：

- 如果一个变量要保持所有权，且又要在其他地方使用的话，可以使用`&`引用一个值，这种行为也称为借用。

正确代码如下：

```rust
fn main() {
    let s = String::from("hello, world");
 -	print_str(s);
 +   print_str(&s);

    println!("{}", s);
}
-fn print_str(s: String)  {
-    println!("{}",s)
-}

+ fn print_str(s: &String)  {
+   println!("{}",s)
+ }
```

5. 🌟🌟

需要注意的是： 

- 在Rust中`to_string`用于将值转换为`String`，因此`"hello".to_string()`将创建一个新的`String`对象

```rust
    let i:i32 = 5;
    let five = String::from("5");
    assert_eq!(five, i.to_string());
```

正确代码如下：

```rust
fn main() {
-	let x = (1, 2, (), "hello".to_string());
+   let x = (1, 2, (), "hello");
-   let y = x.clone();
+    let y = x;
    println!("{:?}, {:?}", x, y);

    let  x = (1, 2, (), "hello".to_string());
- 	let y = x.clone();
+   let  y = & x;
    println!("{:?}, {:?}", x, y);
}
```

6. 🌟

需要注意的是：

- 修改变量需要使用关键字`mut`。

正确代码如下

```rust
fn main() {
    let s = String::from("hello, ");
    // 只修改下面这行代码 !
-    let s1 = s;
+    let mut  s1 =s.clone();
    s1.push_str("world")
}
```

7. 🌟🌟🌟

- `Box`它是一种智能指针，允许将一个值放在堆上，而放在栈上的则是指向堆数据的指针。
- `*`操作符是解引用，它能获取指针或智能指针指向的值。

```rust
    let mut a = Box::new(5);
    *a = 2;
    println!("{}", a)
```

正确代码如下：

```rust
fn main() {
    let x = Box::new(5);
+   let mut y = x.clone();
    *y = 4;
    assert_eq!(*x, 5);
}
```

其中`x.clone()`创建了一个`x`的副本，这个副本也是一个`Box`的智能指针。

8. 🌟

需要注意的是：

- 创建元组：可以使用`()`来创建一个元组，并将不同类型的值放入其中
- 访问元组： 可以使用`.`和索引访问元组的元素，索引从`0`开始。

```rust
    let a = (1,2);
    println!("{}",a.0);// 1
	// 解构
    let (a,b)= (1,2);
    println!("{},{}",a,b); // 1,2
```

正确代码如下： 

```rust
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
-    println!("{:?}", t);
+    println!("{:?}", t.1);
 }
```

9. 🌟🌟

需要注意的是：

- 在Rust中，元素能否复制取决于元素的类型，如果元组的每个元素都实现了`Copy`特征，那么在模式匹配时，元素就能被复制，而不是移动所有权。

```rust
    let a = (2, true, false, 1.0);
    let (s1,s2,s3,s4) = a;
    println!("{:?},{:?},{:?},{:?},{:?}",s1,s2,s3,s4,a);

	// 移动所有权
    let t = (1, String::from("hello"),String::from("world"));
    let (s1,s2,s3) = t;
    println!("{:?},{:?},{:?},{:?}",s1,s2,s3,t);//Error： value borrowed here after partial move
```

正确代码如下：

```rust
fn main() {
    let t = (String::from("hello"),String::from("world"));
-	let (__, __) = __;
+   let (s1, s2) = &t; // 方式1
+	let (s1, s2) = t.clone(); // 方式2
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
```

