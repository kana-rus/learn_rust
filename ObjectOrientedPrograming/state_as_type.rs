struct Post {
    content: String,
} impl Post {

    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

struct DraftPost {
    content: String,
} impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // content() がない：-> draft の中身は公開されない

    pub fn request_review(self) -> PendingReviewPost {
        /* 今の状態の struct を消費して新しい状態の struct を返すので
        所有権を奪って 次の struct を作る */
        PendingReviewPost {
            content: self.content,
        }
    }
}

struct PendingReviewPost {
    content: String,
} impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}


fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}