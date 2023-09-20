### [结构体](https://zh.practice.rs/compound-types/struct.html)

1. 🌟

需要注意的是：

- 使用结构体需要为每个字段指定具体值来创建这个结构体的 **实例**。

- 初始化时的字段顺序不需要和结构体定义时的顺序一致。

正确代码如下：

```rust
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("Cantabile"),
        age,
+        hobby: String::from("code")
    };
} 
```

2. 🌟

需要注意的是：

- 没有任何字段的结构体，也称之为`类单元结构体`，类似于单元类型即`()`。
- 类单元结构体通常用于表示某些类型的存在而不包含任何实际数据。

正确代码如下：

```rust
struct Unit;
trait SomeTrait {
    // ...定义一些行为
}
// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);
} 
- fn do_something_with_unit(u: ) {   }
+ fn do_something_with_unit(u: Unit) {   }
```

3. 🌟🌟🌟

需要注意的是：

- 解构结构体需要使用结构体名称作为标记， 明确指定要解构的数据类型，避免潜在错误。
- `_`在解构元组中作为忽略元素使用。
- 每一个结构体有自己的类型，即使结构体中的字段有相同类型，列如：一个获取 `color`类型参数的函数不能接收`point`作为参数，因为它们是不同的类型，即便类型都为`i32`

```rust
// 元组解构
let tup = (1,2,3);
let (x,y,z) = tup;
println!("{x}");
// 结构体解构
//1:
let Color(_,b,_) = Color(1, 2, 3);
println!("{b}")
//2:
let Person{name, age}  = Person {
    name: String::from("Cantabile"),
    age: 12
};
println!("name is  {}, age is {}", name, age)
```

正确代码如下：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
-    let v = Point(__, __, __);
+    let v = Color(0, 127, 255);
    check_color(v);
}   
fn check_color(p: Color) {  
-    let (x, _, _) = p;
+    let Color(x,_,z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
 }
```

4. 🌟

需要注意的是：

- 整个实例必须是可变的，不允许只将某个字段标记为可变。

正确代码如下：

```rust
 struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
-    let p = Person {
-        name: String::from("sunface"),
-        age,
-    };
    
+    let mut p = Person {
+        name: String::from("sunface"),
+        age,
+    };     
    p.age = 30;
_   __ = String::from("sunfei");
+   p.name = String::from("sunfei");
}
```

5. 🌟

需要注意的是：

- 函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化。

正确代码如下：

```rust
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let result =  build_person(String::from("张三"), 18);
    println!("{}, {}",result.name, result.age);
} 
fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
-       __
+       name
    }
}
```

6. 🌟

需要注意的是：

- `..`结构体更新语法指定了剩余未显示设置值的字段全部获取。
- 结构体更新语法必须在结构体的尾部使用。

正确代码如下：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
+       ..u
    }
}
```

7. 🌟🌟

需要注意的是：

- 使用 `#[derive(Debug)]` 对结构体进行了标记，才能使用 `println!("{:?}", s);`。 
- 需要让打印信息更直观，需要替换`{:?}`为`{:#?}`。

正确代码如下：

```rust
-#[__]
+#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
-   println!(__, rect1);
+   println!("{:#?}", rect1);
}
```

