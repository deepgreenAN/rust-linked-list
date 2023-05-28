use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
    ops::{Index, IndexMut, RangeFrom},
};

use List::{Cons, Nil};

/// リンクトリスト．データはBoxにしているためstd::mem::takeなどのムーブが低コスト．
#[derive(Debug, Clone, Default)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    #[default]
    Nil,
}

impl<T: Default> List<T> {
    /// コンストラクタ
    pub fn new() -> List<T> {
        Nil
    }
    /// 長さを取得O(N)
    pub fn len(&self) -> usize {
        match self {
            Cons(_, tail) => tail.len() + 1,
            Nil => 0,
        }
    }
    /// 空かどうかO(1)
    pub fn is_empty(&self) -> bool {
        match self {
            Cons(_, _) => false,
            Nil => true,
        }
    }
    /// 前に要素を追加．O(1)
    pub fn prepend(&mut self, elem: T) {
        let new_self = Cons(elem, Box::new(std::mem::take(self)));
        *self = new_self
    }
    /// 前の値を取り出す．自身がNilの場合はNoneが返る．
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

    /// 自身のListの最初の値と子Listを取得．自身がNilであった場合はNoneが返る．O(1)
    pub fn first(&self) -> Option<(&T, &List<T>)> {
        match self {
            Cons(head, tail) => Some((head, tail.as_ref())),
            Nil => None,
        }
    }
    /// 自身のListの最初の値と子Listを可変参照で取得．自身がNilであった場合はNoneが返る．O(1)
    pub fn first_mut(&mut self) -> Option<(&mut T, &mut List<T>)> {
        match self {
            Cons(head, tail) => Some((head, tail.as_mut())),
            Nil => None,
        }
    }

    /// `index`番目の要素の参照を取得．O(N)．
    pub fn get(&self, index: usize) -> Option<(&T, &List<T>)> {
        let mut recur_list = self; // 再帰のため
        let mut res_head: Option<&T> = None; // 答えとなるヘッド

        for i in 0..index + 1 {
            match recur_list {
                Cons(head, tail) => {
                    recur_list = tail.as_ref();
                    res_head = Some(head);
                }
                Nil => {
                    // 最後までいかなかった場合．答えはNone
                    if i != index + 1 {
                        res_head = None;
                    }

                    break;
                }
            }
        }
        res_head.map(|head| (head, recur_list))
    }
    /// `index`番目の要素の可変参照を取得．O(N)．
    pub fn get_mut(&mut self, index: usize) -> Option<(&mut T, &mut List<T>)> {
        let mut recur_list = self; // 再帰のため
        let mut res_head: Option<&mut T> = None; // 答えとなるヘッド
        for i in 0..index + 1 {
            match recur_list {
                Cons(head, tail) => {
                    recur_list = tail.as_mut();
                    res_head = Some(head);
                }
                Nil => {
                    // 最後までいかなかった場合．答えはNone
                    if i != index + 1 {
                        res_head = None;
                    }
                    break;
                }
            }
        }
        res_head.map(|head| (head, recur_list))
    }

    /// 自身の後ろ側に`sub_list`を挿入．O(M)．sub_listの要素数が１の場合はO(1)
    pub fn push_next(&mut self, mut sub_list: List<T>) {
        match self.first_mut() {
            Some((_, tail)) => {
                let sub_list_length = sub_list.len();
                // sub_listの最後の値をNilからtailに変更．
                if let Some((_, sub_list_tail_last)) = sub_list.get_mut(sub_list_length - 1) {
                    *sub_list_tail_last = std::mem::take(tail);
                    *tail = sub_list
                }
            }
            None => *self = sub_list,
        }
    }
}

impl<T: Display> List<T> {
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Displayトレイトの実装

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List [{}]", self.stringify())
    }
}

// -------------------------------------------------------------------------------------------------
// PartialEq・Eqトレイトの実装

impl<T: PartialEq + Default> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: Eq + Default> Eq for List<T> {}

// -------------------------------------------------------------------------------------------------
// FromIteratorトレイトの実装

impl<T: Default> FromIterator<T> for List<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut reverse_list: List<T> = Nil;
        for item in iter.into_iter() {
            reverse_list.prepend(item);
        }

        let mut list: List<T> = Nil;
        for item in reverse_list.into_iter() {
            list.prepend(item);
        }
        list
    }
}

// -------------------------------------------------------------------------------------------------
// into_iterの実装

pub struct IntoIter<T: Default>(List<T>);

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T: Default> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

// -------------------------------------------------------------------------------------------------
// iterの実装

pub struct Iter<'a, T: Default>(&'a List<T>);

impl<'a, T: Default> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.first().map(|(head, tail)| {
            self.0 = tail;
            head
        })
    }
}

impl<T: Default> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self)
    }
}

impl<'a, T: Default> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// -------------------------------------------------------------------------------------------------
// iter_mutの実装

pub struct IterMut<'a, T: Default>(Option<&'a mut List<T>>); // takeして&mutから'aにライフタイムを変更する必要がある．

impl<'a, T: Default> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        let list_mut = self.0.take()?;
        let (head, tail) = list_mut.first_mut()?;
        self.0 = Some(tail);
        Some(head)
    }
}

impl<T: Default> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(Some(self))
    }
}

impl<'a, T: Default> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// -------------------------------------------------------------------------------------------------
// indexの実装

impl<T: Default> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if let Some((res, _)) = self.get(index) {
            res
        } else {
            panic!("out of bound.");
        }
    }
}

impl<T: Default> Index<RangeFrom<usize>> for List<T> {
    type Output = List<T>;
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        if index.start == 0 {
            // 長さが0の場合
            self
        } else {
            if let Some((_, res)) = self.get(index.start - 1) {
                res
            } else {
                panic!("out of bound.");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// index_mutの実装

impl<T: Default> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if let Some((res, _)) = self.get_mut(index) {
            res
        } else {
            panic!("out of bound.");
        }
    }
}

impl<T: Default> IndexMut<RangeFrom<usize>> for List<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        if index.start == 0 {
            // 長さが0の場合
            self
        } else {
            if let Some((_, res)) = self.get_mut(index.start - 1) {
                res
            } else {
                panic!("out of bound.");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 作成用マクロ

/// Listを作成する．
#[macro_export]
macro_rules! list {
    // 要素が一つだけの場合
    ($first:expr) => {
        $crate::List::Cons($first, Box::new($crate::List::Nil))
    };
    // 要素が複数の場合(再帰)
    ($first:expr, $($rest:expr),*)  => {
        $crate::List::Cons($first, Box::new($crate::list!($($rest),*)))
    };
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::{list, List};
    use std::panic::catch_unwind;

    #[test]
    fn from_iteration_and_into_iteration_and_macro() {
        let vec = vec![1, 2, 3, 4, 5];
        let vec2 = vec
            .clone()
            .into_iter()
            .collect::<List<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(vec, vec2);

        let list = list![1, 2, 3, 4, 5];
        let vec3 = list.into_iter().collect::<Vec<_>>();

        assert_eq!(vec, vec3);
    }

    #[test]
    fn index_and_panic() {
        let mut list = list![1, 2, 3, 4, 5, 5, 6, 7];

        let list_from_get = (0..list.len())
            .map(|i| list[i].clone())
            .collect::<List<_>>();
        assert_eq!(list, list_from_get);

        // 不適切なインデックスによるパニック
        assert!(catch_unwind(|| { list[list.len()] }).is_err());

        // 最初の値を変更
        list[0] = 100;
        let (first_value, _) = list.first().unwrap();
        assert_eq!(*first_value, 100);
    }

    #[test]
    fn prepend() {
        let mut list = List::<i32>::new();
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);

        assert_eq!(list, list![1, 2, 3]);
    }

    #[test]
    fn pop_front() {
        let mut list = list![1, 2, 3, 4, 5];
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(Some(2), list.pop_front());

        assert_eq!(list, list![3, 4, 5]);
    }

    #[test]
    fn push_next() {
        let mut list = list![1, 2, 3, 7, 8, 9];
        let sub_list = list![4, 5, 6];

        let (_, list_from_3) = list.get_mut(1).unwrap(); // List[3, 7, 8, 9]
        list_from_3.push_next(sub_list); // List[3, 4, 5, 6, 7, 8, 9]

        assert_eq!(list, list![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn iter_mut() {
        let mut list = list![1, 2, 3, 4, 5];

        for item in &mut list {
            *item += 100;
        }

        assert_eq!(list, list![101, 102, 103, 104, 105]);
    }
}
