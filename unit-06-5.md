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

