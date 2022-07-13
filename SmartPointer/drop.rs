struct CustomPointer {
    data: String,
}
impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Drop は初期化処理に含まれるので import する必要がない
    let c = CustomPointer { data: String::from("my stuff") };
    println!("CustomSmartPointers created.");

    /* core::mem */drop(c); // 初期化処理に含まれている
    // これに代えて c.drop() も Drop trait の実装を直接呼び出すが、
    // その場合コンパイラはいつもどおり main の終端で明示的に drop を呼ぼうとする。
    // これは２重解放になるので、予めエラーとなる

    println!("CustomSmartPointer dropped before the end of main.");
}