trait State {  // 「現在の状態を消費して新しい状態を返す」
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""  // デフォルト実装を与える
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // デフォルト実装をオーバーライドする
        &post.content
    }
}


struct Post {
    state: Option<Box<dyn State>>,
    content: String,
} impl Post {

    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state
            .as_ref()
            .unwrap()
            .content(&self)  // このためにも、as_ref で (Option内部の値の) 参照だけを取る必要がある
            // ここで、参照外し型強制で & と Box が外れている
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state
                                             .take() {
            /* take() ... Option の中身を取り出して move する
            take() された側は None になる */
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}


fn main() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}