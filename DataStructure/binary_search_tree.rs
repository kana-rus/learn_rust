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
}
