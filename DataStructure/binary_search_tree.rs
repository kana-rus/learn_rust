/**
 * ristrictions:
 * 1. each node has to have a value that's able to be COMAPRED with other values, or, have 'Nil'.
 * 2. each LEFT child node has to have value SMALLER OR EQUALL to its parent, or, have 'Nil'.
 * 3. each RIGHT child node has to have value LARGER than its parent, or, have 'Nil'.
 */


#[derive(Debug, PartialEq)]
pub struct BinarySearchTree<T: Ord> (
    BinarySearchTreeInner<T>,
); impl<T: Ord> BinarySearchTree<T> {

    pub fn new() -> Self {
        Self(BinarySearchTreeInner::Nil)
    }

    pub fn insert(&mut self, val: T) {
        let nil_to_replace = Self::find_target_nil(&mut self.0, &val);
        *nil_to_replace = BinarySearchTreeInner::Node {
            val,
            left: Box::new(BinarySearchTreeInner::Nil),
            right: Box::new(BinarySearchTreeInner::Nil),
        }
    } fn find_target_nil<'t, 'v>(current_node: &'t mut BinarySearchTreeInner<T>, val: &'v T) -> &'t mut BinarySearchTreeInner<T> {
        match current_node {
            BinarySearchTreeInner::Nil => current_node,
            BinarySearchTreeInner::Node { val: current_val, left, right } => {
                if val <= current_val {
                    Self::find_target_nil(left, &val)
                } else {
                    Self::find_target_nil(right, &val)
                }
            }
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        Self::contains_inner(&self.0, val)
    } fn contains_inner(current_node: &BinarySearchTreeInner<T>, val: &T) -> bool {
        match current_node {
            BinarySearchTreeInner::Nil => false,
            BinarySearchTreeInner::Node { val: current_val, left, right } => {
                if current_val == val {
                    true
                } else {
                    Self::contains_inner(left, val) || Self::contains_inner(right, val)
                }
            }
        }
    }

    pub fn get_all_sorted(&self) -> Vec<&T> {
        let mut sorted = Vec::<&T>::new();
        Self::get_all_sorted_inner(&self.0, &mut sorted);
        sorted
    } fn get_all_sorted_inner<'t, 'a>(current_node: &'t BinarySearchTreeInner<T>, sorted: &'a mut Vec<&'t T>) {
        match current_node {
            BinarySearchTreeInner::Nil => (),
            BinarySearchTreeInner::Node { val, left, right } => {
                Self::get_all_sorted_inner(left, sorted);
                sorted.push(val);
                Self::get_all_sorted_inner(right, sorted);
                /* this order because ristrictions (left =< val < right) */
            }
        }
    }

    pub fn get_ranged_sorted(&self, min: &T, max: &T) -> Vec<&T> {
        let mut ret = Vec::<&T>::new();
        Self::get_ranged_sorted_inner(&self.0, min, max, &mut ret);
        ret
    } fn get_ranged_sorted_inner<'t, 'r>(current_node: &'t BinarySearchTreeInner<T>, min: &T, max: &T, ret: &'r mut Vec<&'t T>) {
        match current_node {
            BinarySearchTreeInner::Nil => (),
            BinarySearchTreeInner::Node { val, left, right } => {
                if val >= min {
                    Self::get_ranged_sorted_inner(left, min, max, ret);
                }
                if min <= val && val <= max {
                    ret.push(&val);
                }
                if val < max {
                    Self::get_ranged_sorted_inner(right, min, max, ret);
                }
                /* this order because ristrictions (left <= val < right) */
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum BinarySearchTreeInner<T: Ord> {
    Nil,
    Node {
        val: T,
        left: Box<Self>,
        right: Box<Self>,
    },
}


fn main() { /* printing debug */    
    println!("\ninsert_in_different_order_1");
    let (mut bst1, mut bst2) = (BinarySearchTree::new(), BinarySearchTree::new());
    bst1.insert(1);
    bst1.insert(2);
    println!("{:?}", bst1);
    bst2.insert(2);
    bst2.insert(1);
    println!("{:?}", bst2);

    println!("\ninsert_in_different_order_2");
    let (mut bst1, mut bst2) = (BinarySearchTree::new(), BinarySearchTree::new());
    bst1.insert(8);
    bst1.insert(5);
    bst1.insert(10);
    bst1.insert(5);
    bst1.insert(3);
    bst1.insert(5);
    bst1.insert(6);
    bst1.insert(8);
    bst1.insert(9);
    bst1.insert(16);
    println!("{:?}", bst1);
    bst2.insert(8);
    bst2.insert(10);
    bst2.insert(5);
    bst2.insert(16);
    bst2.insert(9);
    bst2.insert(6);
    bst2.insert(5);
    bst2.insert(8);
    bst2.insert(3);
    bst2.insert(5);
    println!("{:?}", bst2);
}


#[cfg(test)] mod test {
    use super::BinarySearchTree;

    #[test]
    fn insert_in_same_order() {
        let (mut bst1, mut bst2) = (BinarySearchTree::new(), BinarySearchTree::new());

        bst1.insert(1);
        bst1.insert(2);

        bst2.insert(1);
        bst2.insert(2);

        assert_eq!(bst1, bst2);
    }

    #[test]
    fn insert_in_different_order_1() {
        let (mut bst1, mut bst2) = (BinarySearchTree::new(), BinarySearchTree::new());

        bst1.insert(1);
        bst1.insert(2);

        bst2.insert(2);
        bst2.insert(1);

        assert_ne!(bst1, bst2);
    }

    #[test]
    fn insert_in_different_order_2() {
        let (mut bst1, mut bst2) = (BinarySearchTree::new(), BinarySearchTree::new());

        bst1.insert(8);
        bst1.insert(5);
        bst1.insert(10);
        bst1.insert(5);
        bst1.insert(3);
        bst1.insert(5);
        bst1.insert(6);
        bst1.insert(8);
        bst1.insert(9);
        bst1.insert(16);

        bst2.insert(8);
        bst2.insert(10);
        bst2.insert(5);
        bst2.insert(16);
        bst2.insert(9);
        bst2.insert(6);
        bst2.insert(5);
        bst2.insert(8);
        bst2.insert(3);
        bst2.insert(5);

        /*       8
         *    5    10
         *  5  6   9 16
         * 3    8
         *  5
         * */

        assert_eq!(bst1, bst2);
    }

    #[test]
    fn contains() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(5);
        bst.insert(10);
        bst.insert(5);
        bst.insert(3);
        bst.insert(5);
        bst.insert(6);
        bst.insert(8);
        bst.insert(9);
        bst.insert(16);
        assert!(bst.contains(&8));
        assert!(bst.contains(&10));
        assert!( ! bst.contains(&11));
        assert!(bst.contains(&3));
        assert!( ! bst.contains(&1));
    }

    #[test]
    fn get_all_sorted() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(5);
        bst.insert(10);
        bst.insert(5);
        bst.insert(3);
        bst.insert(5);
        bst.insert(6);
        bst.insert(8);
        bst.insert(9);
        bst.insert(15);
        assert_eq!(
            bst.get_all_sorted(),
            vec![&3, &5, &5, &5, &6, &8, &8, &9, &10, &15]
        );
    }

    #[test]
    fn get_ranged_sorted() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(5);
        bst.insert(10);
        bst.insert(5);
        bst.insert(3);
        bst.insert(5);
        bst.insert(6);
        bst.insert(8);
        bst.insert(9);
        bst.insert(15);
        assert_eq!(
            bst.get_ranged_sorted(&3, &15),
            vec![&3, &5, &5, &5, &6, &8, &8, &9, &10, &15]
        );
        assert_eq!(
            bst.get_ranged_sorted(&4, &14),
            vec![&5, &5, &5, &6, &8, &8, &9, &10]
        );
        assert_eq!(
            bst.get_ranged_sorted(&7, &10),
            vec![&8, &8, &9, &10]
        )
    }
}
