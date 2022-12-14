use std::{
    fmt::Display,
    ops::{Index, IndexMut, RangeFrom},
};

use List::{Cons, Nil};

#[derive(Debug, Clone, Default)]
pub enum List<T: Display + Default> {
    Cons(T, Box<List<T>>),
    #[default]
    Nil,
}

impl<T: Display + Default> List<T> {
    pub fn len(&self) -> usize {
        match self {
            Cons(_, tail) => tail.len() + 1,
            Nil => 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn prepend(self, elem: T) -> List<T> {
        Cons(elem, Box::new(self))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self {
            Cons(head, tail) => {
                let tail_value = std::mem::take(tail);
                let head_value = std::mem::take(head);
                *self = *tail_value;
                Some(head_value)
            }
            Nil => None,
        }
    }
    fn stringfy(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringfy())
            }
            Nil => "Nil".to_string(),
        }
    }
    pub fn split_first(&self) -> Option<(&T, &List<T>)> {
        match self {
            Cons(head, tail) => Some((head, tail.as_ref())),
            Nil => None,
        }
    }
    pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut List<T>)> {
        match self {
            Cons(head, tail) => Some((head, tail.as_mut())),
            Nil => None,
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut list = self;
        let mut res_head: Option<&T> = None;
        for _ in 0..index {
            match list {
                Cons(head, tail) => {
                    list = tail.as_ref();
                    res_head = Some(head);
                }
                Nil => {
                    break;
                }
            }
        }
        res_head
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut list = self;
        let mut res_head: Option<&mut T> = None;
        for _ in 0..index {
            match list {
                Cons(head, tail) => {
                    list = tail.as_mut();
                    res_head = Some(head);
                }
                Nil => {
                    break;
                }
            }
        }
        res_head
    }
    pub fn rest(&self, from: usize) -> &List<T> {
        let mut list = self;
        for _ in 0..from {
            match list {
                Cons(_, tail) => {
                    list = tail.as_ref();
                }
                Nil => {
                    break;
                }
            }
        }
        list
    }
    pub fn rest_mut(&mut self, from: usize) -> &mut List<T> {
        let mut list = self;
        for _ in 0..from {
            match list {
                Cons(_, tail) => {
                    list = tail.as_mut();
                }
                Nil => {
                    break;
                }
            }
        }
        list
    }
}

impl<T: Display + Default> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List [{}]", self.stringfy())
    }
}

// -------------------------------------------------------------------------------------------------
// FromIterator?????????????????????

impl<T: Display + Default> FromIterator<T> for List<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut list: List<T> = Nil;
        for item in iter.into_iter() {
            list = list.prepend(item);
        }
        list
    }
}

// -------------------------------------------------------------------------------------------------
// into_iter?????????

pub struct IntoIter<T: Display + Default>(List<T>);

impl<T: Display + Default> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T: Display + Default> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

// -------------------------------------------------------------------------------------------------
// iter?????????

pub struct Iter<'a, T: Display + Default>(&'a List<T>);

impl<'a, T: Display + Default> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.split_first().map(|(head, tail)| {
            self.0 = tail;
            head
        })
    }
}

impl<T: Display + Default> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self)
    }
}

impl<'a, T: Display + Default> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// -------------------------------------------------------------------------------------------------
// index?????????

impl<T: Display + Default> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if let Some(res) = self.get(index) {
            res
        } else {
            panic!("out of bound.");
        }
    }
}

impl<T: Display + Default> Index<RangeFrom<usize>> for List<T> {
    type Output = List<T>;
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.rest(index.start)
    }
}

// -------------------------------------------------------------------------------------------------
// index_mut?????????

impl<T: Display + Default> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let Some(res) = self.get_mut(index) {
            res
        } else {
            panic!("out of bound.");
        }
    }
}

impl<T: Display + Default> IndexMut<RangeFrom<usize>> for List<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        self.rest_mut(index.start)
    }
}

fn main() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list = list.prepend(0);
    println!("list length: {}", list.len());
    println!("list :{}", list);

    println!("pop_front: {:?}, list: {}", list.pop_front(), list);

    let mut list_from_vec = vec![11, 12, 13, 14].into_iter().collect::<List<_>>();
    println!("list from vec:{}", list_from_vec);
    println!("list[2]: {}", list_from_vec[2]);
    list_from_vec[3] = 3;
    println!("list from vec:{}", list_from_vec);
    println!("rest from 2: {}", list_from_vec[2..]);
    list_from_vec[2..] = Cons(300, Box::new(Cons(200, Box::new(Cons(100, Box::new(Nil))))));
    println!("list from vec:{}", list_from_vec);
}
