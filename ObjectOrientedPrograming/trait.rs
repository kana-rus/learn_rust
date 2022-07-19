trait Draw {
    fn draw(&self);  // &, Box など、なにかしらのポインタ型を引数にとる
}

struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("new Button: {}x{}, \"{}\"", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("new SelectBox: {}x{}, \"{:?}\"", self.width, self.height, self.options);
    }
}


// Generics を使う
struct GenericsScreen<T: Draw> {
    pub components: Vec<Box<T>>,
} impl<T> GenericsScreen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main_generics() {
    let screen = GenericsScreen { components: vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
/*
    １つの Vec には１種の型の要素しか入れないので
    ここでエラー
*/
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ]};

    screen.run();
}


/* trait オブジェクトを使う */
struct Screen {
    components: Vec<Box<dyn Draw>>,  // *
} impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
/**
 * - Generics はコンパイル時にダイナミックディスパッチにより具体的な型を使うコードを生成する
 * - trait オブジェクトはそれができないため実行時パフォーマンスを犠牲にする
 * 
 * - trait オブジェクトとして指定される trait は「オブジェクト安全」でなければならない
 * - オブジェクト安全：
 *   - メソッドが Self を返さない
 *   - メソッドが Generics を使わない
 * - 例として、Clone trait は
 *      fn clone(&self) -> Self
 *   というメソッドを持つので、
 *      struct Hogehoge {
 *          members: Vec<Box<Clone>>
 *      }
 *   のように指定するとエラーになる
 */
fn main() {
    let screen = Screen { components: vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
/*
    Vec の中を trait オブジェクトで定義することで、
    その trait を満たす任意の型の値を Vec の要素として許容させる
*/
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ]};

    screen.run();
}
