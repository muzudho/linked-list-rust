pub mod iter;
pub mod linked_list;
pub mod node;

use std::cell::RefCell;
use std::rc::Rc;

/// Linked List を作るには工夫が必要です。
/// [Rustでdoubly linked list](https://blog.ymgyt.io/entry/2019/08/17/013313)
///
/// `Rc` - 参照の所有者を複数にします。
/// `RefCell` - 参照先からでも値が変更できるようにします。
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

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub prev: Option<Link<T>>,
    pub next: Option<Link<T>>,
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
