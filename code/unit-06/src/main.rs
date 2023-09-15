/* 1 
fn main() {
    let s: &str = "hello, world";
}
*/

/* 2.
// 1 
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

// 2: 
fn main() {
    let s: Box<str> = "hello, world".into();
    println!("{s}")
    greetings(s)
}

fn greetings(s: Box<str>) {
    println!("{}",s)
}
 */

/* 3: 
fn main() {
    let mut s = String::with_capacity(10);
    let mut s = String::from("");
    let mut s = "".to_string();
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    println!("{s}");
    assert_eq!(s, "hello, world!");
}

*/

/* 4. 
fn main() {
    // let mut s = String::from("hello");
    // s.push_str(&','.to_string());
    // s.push_str(" world");
    // s += &"!".to_string();
    // println!("{}", s)

    // let mut s = String::from("hello");
    // s += &','.to_string();
    // s += &" world";
    // s += &"!".to_string();
    // println!("{}", s)

    let mut  s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}
*/

/* 5.
fn main() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
} */

/* 6.
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
} */

/* 7. 

fn main() {
    let s = "hello, world".into();
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}

fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
*/

/* 8. 
fn main() {
    // 1: 
    let s = "hello, world".to_string();
    let s1: &str = &s;
    println!("{s1}")

    // 2: 
    let s = "hello, world";
    let s1: &str = s;
    println!("{s1}")
}
*/

/* 9. 
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
}*/

/* 10 
fn main() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!"#;
    println!("{}", quotes);

    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    let long_delimiter = "Hello, \"##\"";
    let long_delimiter = r###"Hello, "##""###;
    println!("{long_delimiter}");
    assert_eq!(long_delimiter, "Hello, \"##\"")
}*/

/* 11. 
fn main() {
    // let s1 = String::from("hi,中国");
    // let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    // assert_eq!(h, "h");

    // let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    // assert_eq!(h1, "中");

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
}
*/

/* 12 */
