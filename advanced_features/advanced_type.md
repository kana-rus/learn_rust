## type alias のうまい使い方例
- Error としてほとんど毎回特定の１種類 (例として `std::io::Error`)
  のみがでてくる場合、以下の alias を宣言しておく

```rs
tyoe Result<T> = Result<T, std::io::Error>;
```


<br>
<br>


## never型 " ! "
```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

という実装があるとして、`continue` は明らかに `u32` 型ではないが、
これは正しい Rust コード。
この `continue` や

- エラーハンドリングを放棄する `panic()`,
- 条件が成り立たなければプログラムを停止する `assert!()` 系のマクロ

などに登場する、「決して値を返さない」ことを表す型を「`never`型」といい、
直接扱うときは
    `!`
で表す。  
型理論の専門用語では Empty型といい、これを返り値の型とする関数を「発散する関数」と呼ぶ  


<br>
<br>


## トレイト境界の細かい指定
Rust の Generics は、考えてみると当たり前だが、標準で `Sized` を要求する。
そこで、`Sized` を積んでいない型も積んでいる型も扱いたい場合のために、`Sized` に限って
```rs
fn generic<T: ?Sized>(t: &T) {
    /* T は Sized を積んでいてもいなくてもいいよ */
}
```
という記法が存在する。また、一般に
```rs
fn generic<T: !Sized>(t: &T) {
    /* T は Sized を積んでいてはならない */
}
```
という指定ができる。


<br>
<br>


## クロージャーを返す
関数の返り値として closure を指定することは当然できないが、これの原因は **クロージャーが Sized を積んでいない** ことに集約される。  
よって、実は以下のようにすれば実質的に関数で closure を返すことができる。
```rs
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(
        |x| x + 1
    )
}
```