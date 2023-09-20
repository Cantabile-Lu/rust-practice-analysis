/* 1: 
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
        hobby: String::from("code")
    };
} 
*/

/* 2:
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

// 填空，让代码工作
fn do_something_with_unit(u: Unit) {   }
 */

 /* 3: 

 // 填空并修复错误
// struct Color(i32, i32, i32);
// struct Person {
//     name: String,
//     age: i8
// }
// fn main() {
//     let Color(_,b,_) = Color(0, 127, 255);
//     println!("{b}");
    
//     let Person{name,..}  = Person {
//         name: String::from("Cantabile"),
//         age: 12
//     };
//     println!("name is  {}", name)
    
// }   
struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
fn main() {
    let v = Color(0, 127, 255);
    check_color(v);
}   

fn check_color(p: Color) {
    let Color(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }*/

 /* 4: 
 struct Person {
    name: String,
    age: u8,
}
fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };
    p.age = 30;
    // 填空
    p.name = String::from("sunfei");
}*/

/* 5: 
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
        name
    }
}*/

/* 6: */
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
        ..u
    }
}
