use std::cmp::Ord;
use std::cmp::Ordering;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST::Empty
    }

    pub fn create(value: T) -> Self {
        BST::Leaf {
            value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }

    pub fn insert(&mut self, new_value: T) {
        print_type_of(&(new_value));
        //println!("long tuple first value: {}", new_value.0);
        match self {
            BST::Leaf {
                ref value,
                ref mut left,
                ref mut right,
            } => match new_value.cmp(value) {
                Ordering::Less => left.insert(new_value),
                Ordering::Greater => right.insert(new_value),
                _ => {}
            },
            BST::Empty => {
                *self = BST::create(new_value);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BST::Empty => true,
            BST::Leaf { .. } => false,
        }
    }

    pub fn find(&self, find_value: T) -> bool {
        match self {
            BST::Leaf {
                ref value,
                ref left,
                ref right,
            } => match find_value.cmp(value) {
                Ordering::Less => left.find(find_value),
                Ordering::Greater => right.find(find_value),
                Ordering::Equal => true,
            },
            BST::Empty => false,
        }
    }
}

impl<T: Ord> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}


fn main(){
    let mut t1 = BST::new();
    t1.insert((3, 1, 4));
    t1.insert((2, 1, 2));
    t1.insert((4, 1, 3));
    assert_eq!(true, t1.find((4, 1, 3)));
    assert_eq!(false, t1.find((5, 5, 5)));
    println!("{:?}", t1)
}
