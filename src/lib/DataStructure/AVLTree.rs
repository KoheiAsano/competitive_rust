#[derive(Debug, Clone)]
pub struct BST<T: PartialEq + PartialOrd + Clone + std::fmt::Debug> {
    h: isize,
    v: Option<T>,
    l: Option<Box<BST<T>>>,
    r: Option<Box<BST<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone + std::fmt::Debug> BST<T> {
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
        if let Some(v) = &self.v {
            if let Some(l) = &self.l {
                if let Some(_v) = &l.v {
                    l.min()
                } else {
                    Some((*v).clone())
                }
            } else {
                Some((*v).clone())
            }
        } else {
            unreachable!();
        }
    }

    // 左部分木の高さ-右部分木の高さ
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

    fn fix_height(&mut self) {
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
    fn rotate_right(&mut self) {
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
            lst.fix_height();
            // 5. 元左部分木(新親)の右に元親を接ぐ
            self.r = Some(lst);
            self.fix_height();
        } else {
            ()
        }
    }

    fn rotate_left(&mut self) {
        if self.r.is_some() {
            let mut rst: Box<BST<T>> = self.r.take().unwrap();
            let rst_lst: Option<Box<BST<T>>> = if rst.l.is_some() { rst.l.take() } else { None };
            std::mem::swap(&mut *self, &mut rst);
            rst.r = rst_lst;
            rst.fix_height();
            self.l = Some(rst);
            self.fix_height();
        } else {
            ()
        }
    }

    fn rotate_left_right(&mut self) {
        if let Some(l) = &mut self.l {
            l.rotate_left();
            self.rotate_right();
        }
    }

    fn rotate_right_left(&mut self) {
        if let Some(r) = &mut self.r {
            r.rotate_right();
            self.rotate_left();
        }
    }

    pub fn new() -> BST<T> {
        BST {
            h: 0,
            v: None,
            l: None,
            r: None,
        }
    }

    pub fn find(&self, n: &T) -> bool {
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
    pub fn delete(&mut self, n: &T) {
        if let Some(v) = &self.v {
            if *v == *n {
                if let (Some(_l), Some(r)) = (&mut self.l, &mut self.r) {
                    let m = r.min();
                    r.delete(m.as_ref().unwrap());
                    self.v = m;
                } else if self.l.is_some() {
                    let l = self.l.take().unwrap();
                    *self = *l;
                } else if self.r.is_some() {
                    let r = self.r.take().unwrap();
                    *self = *r;
                } else {
                    self.v = None;
                }
            } else if *v < *n {
                match &mut self.r {
                    Some(r) => r.delete(n),
                    None => (),
                }
                if self.bias() == 2 {
                    match &mut self.l {
                        Some(l) => {
                            if l.bias() == -1 {
                                self.rotate_right_left();
                            } else {
                                self.rotate_right();
                            }
                        }
                        None => unreachable!(),
                    }
                }
            } else {
                match &mut self.l {
                    Some(l) => l.delete(n),
                    None => (),
                }
                if self.bias() == 2 {
                    match &mut self.r {
                        Some(r) => {
                            if r.bias() == 1 {
                                self.rotate_left_right();
                            } else {
                                self.rotate_left();
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        } else {
            ()
        }
    }

    pub fn insert(&mut self, n: T) {
        if let Some(v) = &self.v {
            if *v < n {
                match &mut self.r {
                    Some(r) => r.insert(n),
                    None => self.r = Some(BST::new_node(n)),
                }
                self.fix_height();
                if self.bias() == -2 {
                    match &self.r {
                        Some(r) => {
                            if r.bias() == -1 {
                                self.rotate_left();
                            } else {
                                self.rotate_right_left();
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
                self.fix_height();
                if self.bias() == 2 {
                    match &self.l {
                        Some(l) => {
                            if l.bias() == 1 {
                                self.rotate_right();
                            } else {
                                self.rotate_left_right();
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
    println!("{:?}", bst);
    bst.insert(4);
    println!("{:?}", bst);
    bst.insert(5);
    println!("{:?}", bst);
    bst.insert(6);
    println!("{:?}", bst);
    let x = 1;
    let y = String::new();
    let _yp = String::new();
    match (x, &y) {
        (1, _yp) => println!("{:?}", (x, y)),
        _ => println!("{:?}", "NO"),
    }
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
