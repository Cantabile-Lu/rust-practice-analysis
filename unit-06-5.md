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

