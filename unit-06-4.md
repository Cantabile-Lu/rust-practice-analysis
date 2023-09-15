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



正确代码如下：