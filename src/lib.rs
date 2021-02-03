use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Node<T: Ord + Debug> {
    Exist { val: T, freq: i32, left: Box<Node<T>>, right: Box<Node<T>> },
    Empty,
}

impl<'a: 'b, 'b, T: Ord + Debug> Node<T> {
    pub fn new() -> Node<T> {
        Node::Empty
    }

    pub fn insert(&mut self, new_val: T) {
        let mut target = self;
        loop {
            match target {
                &mut Node::Empty => {
                    *target = Node::Exist { val: new_val, freq: 1, left: Box::new(Node::Empty), right: Box::new(Node::Empty) };
                    return;
                },
                &mut Node::Exist { ref val, ref mut freq, ref mut left, ref mut right } => {
                    target = if new_val < *val {
                        &mut *left
                    } else if new_val > *val {
                        &mut *right
                    } else {
                        *freq += 1;
                        return;
                    };
                }
            }
        }
    }

    pub fn search(&self, search_val: &T) -> &Node<T> {
        let mut target = self;
        loop {
            match target {
                &Node::Empty => {
                    break;
                },
                &Node::Exist { ref val, freq: _, ref left, ref right } => {
                    target = if *search_val < *val {
                        &*left
                    } else if *search_val > *val {
                        &*right
                    } else {
                        break;
                    };
                }
            }
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert01() {
        let mut tree = Node::new();

        tree.insert("aaa");
        tree.insert("bbb");
        tree.insert("ccc");

        let estimated_result = Node::Exist { val: "aaa", freq: 1,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Exist { val: "bbb", freq: 1,
                left: Box::new(Node::Empty),
                right: Box::new(Node::Exist { val: "ccc", freq: 1,
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty)
                })
            })
        };

        assert_eq!(tree, estimated_result);
    }

    #[test]
    fn test_insert02() {
        let mut tree = Node::new();

        tree.insert(5);
        tree.insert(2);
        tree.insert(7);
        tree.insert(1);
        tree.insert(6);

        let estimated_result = Node::Exist { val: 5, freq: 1,
            left: Box::new(Node::Exist { val: 2, freq: 1,
                left: Box::new(Node::Exist { val: 1, freq: 1,
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty)
                }),
                right: Box::new(Node::Empty)
            }),
            right: Box::new(Node::Exist { val: 7, freq: 1,
                left: Box::new(Node::Exist { val: 6, freq: 1,
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty)
                }),
                right: Box::new(Node::Empty)
            })
        };

        assert_eq!(tree, estimated_result);
    }

    #[test]
    fn test_insert03() {
        let mut tree = Node::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        let estimated_result = Node::Exist { val: 1, freq: 2,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Exist { val: 2, freq: 2,
                left: Box::new(Node::Empty),
                right: Box::new(Node::Exist { val: 3, freq: 1,
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty)
                })
            })
        };

        assert_eq!(tree, estimated_result);
    }

    #[test]
    fn test_search01() {
        let mut tree = Node::new();
        
        tree.insert("bbb");
        tree.insert("aaa");
        tree.insert("ccc");
        tree.insert("zzz");
        tree.insert("ddd");

        let result = tree.search(&"ccc");

        let estimated_result = Node::Exist { val: "ccc", freq: 1,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Exist { val: "zzz", freq: 1,
                left: Box::new(Node::Exist { val: "ddd", freq: 1,
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty)
                }),
                right: Box::new(Node::Empty)
            })
        };

        assert_eq!(*result, estimated_result);
    }

    #[test]
    fn test_search02() {
        let mut tree = Node::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(2);
        tree.insert(2);

        let result = tree.search(&2);

        let estimated_result = Node::Exist { val: 2, freq: 3,
            left: Box::new(Node::Empty),
            right: Box::new(Node::Empty)
        };

        assert_eq!(*result, estimated_result);
    }

    #[test]
    fn test_search03() {
        let mut tree = Node::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);

        let result = tree.search(&6);

        let estimated_result = Node::Empty;

        assert_eq!(*result, estimated_result);
    }
}
