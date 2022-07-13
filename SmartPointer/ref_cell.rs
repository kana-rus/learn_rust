pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
} impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let per_of_max = (self.value as f64) / (self.max as f64);
        
        if 0.75 <= per_of_max {
            self.messenger.send(
                if per_of_max < 0.9 {
                    "Warning: You've used up over 75% of your quota!"
                } else if per_of_max < 1.0 {
                    "Urgent warning: You've used up over 90% of your quota!"
                } else {
                    "Error: You are over your quota!"
                }
            )
        } 
    }
}


fn main() {
    
}