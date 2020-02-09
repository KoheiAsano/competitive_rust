#[derive(Debug, Clone)]
struct BST<T: PartialEq + PartialOrd + Clone> {
    h: isize,
    v: Option<T>,
    l: Option<Box<BST<T>>>,
    r: Option<Box<BST<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone> BST<T> {
    fn new() -> BST<T> {
        BST {
            h: 0,
            v: None,
            l: None,
            r: None,
        }
    }

    fn new_node(n: T) -> Box<BST<T>> {
        Box::new(BST {
            h: 0,
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

    fn bias(&self) -> isize {
        if let (Some(l), Some(r)) = (&self.l, &self.r) {
            l.h - r.h
        } else if let Some(l) = &self.l {
            l.h + 1
        } else if let Some(r) = &self.r {
            -r.h - 1
        } else {
            0
        }
    }

    fn fixHeight(&mut self) {
        if let (Some(l), Some(r)) = (&self.l, &self.r) {
            self.h = std::cmp::max(l.h, r.h) + 1
        } else if let Some(l) = &self.l {
            self.h = l.h + 1
        } else if let Some(r) = &self.r {
            self.h = r.h + 1
        } else {
            self.h = 0
        }
    }

    // 1. 親の左部分木をもぐ
    // 2. もいだ左部分木の右部分木をもぐ
    // 3. もいだ左部分木を親にする
    // 4. 元親の左部分木に元左の右を接ぐ
    // 5. 元左部分木(新親)の右に元親を接ぐ
    fn rotateR(&mut self) {
        // 左の子がなければ右回転できない
        if self.l.is_some() {
            // 1. 親の左部分木をもぐ
            let mut lst: Box<BST<T>> = self.l.take().unwrap();
            // 2. もいだ左部分木の右部分木をもぐ
            let lst_rst: Option<Box<BST<T>>> = if lst.r.is_some() { lst.r.take() } else { None };
            // 3. もいだ左部分木を親にする
            std::mem::swap(&mut *self, &mut lst);
            // 4. 元親の左部分木に元左の右を接ぐ
            lst.l = lst_rst;
            lst.fixHeight();
            // 5. 元左部分木(新親)の右に元親を接ぐ
            self.r = Some(lst);
            self.fixHeight();
        } else {
            ()
        }
    }

    fn rotateL(&mut self) {
        if self.r.is_some() {
            let mut rst: Box<BST<T>> = self.r.take().unwrap();
            let rst_lst: Option<Box<BST<T>>> = if rst.l.is_some() { rst.l.take() } else { None };
            std::mem::swap(&mut *self, &mut rst);
            rst.r = rst_lst;
            rst.fixHeight();
            self.l = Some(rst);
            self.fixHeight();
        } else {
            ()
        }
    }

    fn rotateLR(&mut self) {
        if let Some(l) = &mut self.l {
            l.rotateL();
            self.rotateR();
        }
    }

    fn rotateRL(&mut self) {
        if let Some(r) = &mut self.r {
            r.rotateR();
            self.rotateL();
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
                    let l = self.l.take().unwrap();
                    *self = *l;
                } else if self.r.is_some() {
                    let r = self.r.take().unwrap();
                    *self = *r;
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
                    None => self.r = Some(BST::new_node(n)),
                }
                self.fixHeight();
                if self.bias() == -2 {
                    match &mut self.r {
                        Some(r) => {
                            if r.bias() == -1 {
                                self.rotateL();
                            } else {
                                self.rotateRL();
                            }
                        }
                        None => unreachable!(),
                    }
                }
            } else {
                match &mut self.l {
                    Some(l) => l.insert(n),
                    None => self.l = Some(BST::new_node(n)),
                }
                self.fixHeight();
                if self.bias() == 2 {
                    match &mut self.l {
                        Some(l) => {
                            if l.bias() == 1 {
                                self.rotateR();
                            } else {
                                self.rotateLR();
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        } else {
            self.v = Some(n);
        }
    }
}

#[test]
fn check_insert() {
    let mut bst = BST::new();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    println!("{:?}", bst.bias());
    println!("{:?}", bst);
}

#[test]
fn check_rotate() {
    let mut bst = BST::new();
    bst.insert(7);
    bst.insert(3);
    bst.insert(8);
    bst.insert(5);
    bst.insert(4);
    bst.insert(6);
    println!("{:?}", bst);
    bst.rotateR();
    println!("{:?}", bst);
    bst.rotateL();
    println!("{:?}", bst);
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
