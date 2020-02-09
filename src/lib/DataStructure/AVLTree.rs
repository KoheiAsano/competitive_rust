#[derive(Debug, Clone)]
struct BST<T: PartialEq + PartialOrd + Clone> {
    d: isize,
    v: Option<T>,
    l: Option<Box<BST<T>>>,
    r: Option<Box<BST<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone> BST<T> {
    fn new() -> BST<T> {
        BST {
            d: 0,
            v: None,
            l: None,
            r: None,
        }
    }

    fn new_node(n: T) -> Box<BST<T>> {
        Box::new(BST {
            d: 0,
            v: Some(n),
            l: None,
            r: None,
        })
    }
    // cloneする？
    fn min(&self) -> Option<T> {
        if let Some(l) = &self.l {
            l.min()
        } else {
            if let Some(v) = &self.v {
                Some((*v).clone())
            } else {
                None
            }
        }
    }

    fn find(&self, n: &T) -> bool {
        if let Some(v) = &self.v {
            if *v == *n {
                true
            } else if *v < *n {
                match &self.r {
                    Some(r) => r.find(n),
                    None => false,
                }
            } else {
                match &self.l {
                    Some(l) => l.find(n),
                    None => false,
                }
            }
        } else {
            false
        }
    }
    // 存在しない要素に対してと空の木に対してはなにもしない
    fn delete(&mut self, n: &T) {
        if let Some(v) = &self.v {
            if *v == *n {
                if let (Some(_l), Some(r)) = (&mut self.l, &mut self.r) {
                    let m = r.min();
                    self.delete(m.as_ref().unwrap());
                    self.v = m;
                } else if self.l.is_some() {
                    let l = self.l.take();
                    *self = *(l.unwrap());
                } else if self.r.is_some() {
                    let r = self.r.take();
                    *self = *(r.unwrap());
                } else {
                    ()
                }
            } else if *v < *n {
                match &mut self.r {
                    Some(r) => r.delete(n),
                    None => (),
                }
            } else {
                match &mut self.l {
                    Some(l) => l.delete(n),
                    None => (),
                }
            }
        } else {
            ()
        }
    }

    fn insert(&mut self, n: T) {
        if let Some(v) = &self.v {
            if *v < n {
                match &mut self.r {
                    Some(r) => r.insert(n),
                    None => {
                        self.r = Some(BST::new_node(n));
                        if self.l.is_none() {
                            self.d += 1;
                        }
                    }
                }
            } else {
                match &mut self.l {
                    Some(l) => l.insert(n),
                    None => {
                        self.l = Some(BST::new_node(n));
                        if self.r.is_none() {
                            self.d += 1;
                        }
                    }
                }
            }
        } else {
            self.v = Some(n);
        }
    }
}

#[test]
fn check_int() {
    let mut bst = BST::new();
    bst.insert(5);
    bst.insert(-5);
    bst.insert(-10);
    bst.insert(12);
    assert_eq!(bst.find(&(-5)), true);
    assert_eq!(bst.find(&(-7)), false);
}
#[test]
fn check_string() {
    let mut bst = BST::new();
    bst.insert(String::from("l"));
    bst.insert(String::from("a"));
    bst.insert(String::from("y"));
    bst.insert(String::from("x"));
    bst.insert(String::from("z"));
    assert_eq!(bst.find(&String::from("z")), true);
    assert_eq!(bst.find(&String::from("d")), false);
    bst.delete(&String::from("l"));
    assert_eq!(bst.find(&String::from("l")), false);
    assert_eq!(bst.find(&String::from("x")), true);
}

fn main() {}
