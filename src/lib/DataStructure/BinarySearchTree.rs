#[derive(Debug, Clone)]
enum MBST {
    Lf,
    Br {
        l: Box<MBST>,
        v: usize,
        r: Box<MBST>,
    },
}

impl MBST {
    fn insert(&mut self, k: usize) {
        match self {
            MBST::Lf => {
                *self = MBST::Br {
                    l: Box::new(MBST::Lf),
                    v: k,
                    r: Box::new(MBST::Lf),
                }
            }
            MBST::Br { l, v, r } => {
                if *v <= k {
                    r.insert(k);
                } else if *v > k {
                    l.insert(k)
                }
            }
        }
    }

    fn max(&self) -> usize {
        match self {
            MBST::Lf => 0,
            MBST::Br { v, r, .. } => match **r {
                MBST::Lf => *v,
                MBST::Br { .. } => r.max(),
            },
        }
    }

    fn min(&self) -> usize {
        match self {
            MBST::Lf => std::usize::MAX,
            MBST::Br { l, v, .. } => match **l {
                MBST::Lf => *v,
                MBST::Br { .. } => l.min(),
            },
        }
    }

    fn remove(&mut self, k: usize) {
        match self {
            MBST::Lf => (),
            MBST::Br { l, v, r } => {
                if *v == k {
                    match **l {
                        MBST::Lf => match **r {
                            MBST::Lf => *self = MBST::Lf,
                            MBST::Br { .. } => {
                                let rt = (**r).clone();
                                **r = MBST::Lf;
                                *self = rt;
                            }
                        },
                        MBST::Br { .. } => match **r {
                            MBST::Lf => {
                                let lt = (**l).clone();
                                **l = MBST::Lf;
                                *self = lt;
                            }
                            MBST::Br { .. } => {
                                let lm = l.max();
                                *v = lm;
                                l.remove(lm);
                            }
                        },
                    }
                } else if *v < k {
                    r.remove(k)
                } else {
                    l.remove(k)
                }
            }
        }
    }
    fn inorder_not_recursive(&self) {
        let mut stack = vec![];
        let mut cur = self;
        loop {
            loop {
                match cur {
                    MBST::Lf => break,
                    MBST::Br { l, .. } => {
                        stack.push(cur);
                        cur = &l;
                    }
                }
            }
            match cur {
                MBST::Lf => loop {
                    if stack.len() == 0 {
                        return;
                    }
                    cur = stack.pop().expect("wrong");
                    match cur {
                        MBST::Lf => unreachable!(),
                        MBST::Br { v, r, .. } => {
                            println!("{}", v);
                            match **r {
                                MBST::Lf => continue,
                                MBST::Br { .. } => {
                                    cur = r;
                                    break;
                                }
                            }
                        }
                    }
                },
                MBST::Br { .. } => unreachable!(),
            }
        }
    }
}

#[test]
fn mbst_test() {
    let mut mbst = MBST::Lf;
    mbst.insert(4);
    mbst.insert(4);
    mbst.insert(0);
    mbst.insert(3);
    mbst.insert(1);
    println!("{:?}", mbst);
    println!("{:?}", mbst.max());
    println!("{:?}", mbst.min());
    mbst.remove(0);
    mbst.remove(4);
    println!("{:?}", mbst.min());
    println!("{:?}", mbst.max());
    mbst.remove(4);
    println!("{:?}", mbst.max());
}
