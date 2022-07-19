/*
 * 関連型について
 * 
 * impl Iterator {
 *     type Item;
 *     
 &*     fn next(&mut self) -> Option<Self::Item>
 * }
 * 
 * の Item のようの関連型は Generics と似ているが、Genrics だと
 * 使う側が毎回型を指定しないといけない点で面倒
 */




use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: (self.x + other.x),
            y: (self.y + other.y),
        }
    }
}

fn operation_overload_demo() {
    println!("{:?}",
        Point{ x: 1, y: 2 } + Point { x: 3, y: 4 }
    );  // Point { x: 4, y: 6 }
    // Rust 組み込みの + の定義をオーバーロードしている
}

/* ここで Add trait は
 *
 * trait Add<RHS=Self> {
 *     type Output;
 * 
 *     fn add(self, rhs: RHS) -> Self::Output;
 * }
 *
 * となっている。<RHS=Self> はデフォルト「型引数」
 */
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters { // RHS = Meters と指定
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + 1000 * rhs.0)
    }
}

fn impl_add_with_type_param_demo() {
    println!("{:?}",
        Millimeters(50) + Meters(1)
    );  // Millimeters(1050)
}




/*
 * フルパス記法
 * 
 * trait Alphabet {
 *     fn name();
 * }
 * 
 * struct A;
 * impl Alphabet for A {
 *     fn name() {
 *         println("I am A");
 *     }
 * }
 * 
 * struct B;
 * impl Alphabet for B {
 *     fn name() {
 *         println("I am B");
 *     }
 * }
 *
 * のような状況で A の name() 実装を使いたい場合
 * 
 * fn main() {
 *     <A as Alphabet>::name();
 * }
 * 
 * と書ける
 */



use std::fmt::{self, Display};

trait OutlinePrint: Display {
    /* fmt::Display をスーパートレイトとして要求している */
    fn outline_print(&self) {  // 「Self には Display が実装されている」ことを前提とする
        let output = self.to_string();  // Display を積んでいるので to_string() を呼べる
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Point ... 上で定義した Point { x: i32, y: i32 }
/*
  Display を積んでいなかったので、このままでは当然
      impl OutlinePrint for Point
  と書こうとした時点で違法
*/
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)  // Result<()>
    }
}
impl OutlinePrint for Point {}  // use default implementation

fn super_trait_demo() {
    let point = Point { x: 255, y: 32 };
    point.outline_print();
}




struct Wrapper(Vec<String>);
impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn new_type_pattern_demo() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}




fn main() {
    operation_overload_demo();
    impl_add_with_type_param_demo();
    super_trait_demo();
    new_type_pattern_demo();
}