pub mod iter;
pub mod linked_list;
pub mod node;

use crate::node::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Default)]
pub struct LinkedList<T> {
    /// 先頭ノードです。
    head: Option<Link<T>>,
    /// 末端ノードです。
    tail: Option<Link<T>>,
    /// ノード数です。
    length: usize,
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    /// イントゥ・イテレーターを返します。
    fn into_iter(self) -> Self::IntoIter {
        // イテレーターを返すのと同じ振る舞いで実装します。
        self.iter()
    }
}

/// カレント（現在）だけを持っています。
/// リンクトリストなので、配列のようなインデックスはありません。
pub struct Iter<T> {
    current: Option<Link<T>>,
}
