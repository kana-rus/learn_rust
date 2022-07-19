use core::slice;

fn _split_at_mut_sample() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn _my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);

    /*
        (&mut slice[..mid], &mut slice[mid..])
    とすると、「１つのスライスから２つ参照をとっていても、
    参照部分がけっして被らないので問題ない」ということを
    コンパイラが理解できないためエラーになる
    */

    let ptr = slice.as_mut_ptr();
    unsafe {  // slice::from_raw_parts_mut, offset を呼ぶために unsafe
        // assert(mid <= len) が safety の保証
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}




static mut COUNTER: i32 = 0;
fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;  // データ競合が起こる可能性を排除できない
    }
}

fn main() {
    unsafe {  // println! 含め以下全て unsafe
        println!("{}", &COUNTER);
        add_to_count(3);
        println!("{}", COUNTER);
    }
}