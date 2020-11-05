use crate::{Iter, LinkedList, Node};
use std::fmt;
use std::rc::Rc;

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    /// リンクトリストの末尾に、対象を追加します。
    pub fn append(&mut self, v: T) {
        let node = Node::new(v);
        // tail を空っぽにします。中身は消えていないので、中身の所有者は増減しません。
        match self.tail.take() {
            Some(old_tail) => {
                // さっき tail に入っていたものが、 old_tail です。
                // `RC::clone( )` - 所有者が増えました。
                old_tail.borrow_mut().next = Some(Rc::clone(&node));
                // 新しい対象の prevリンク に、 old_tail をセットします。
                node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                // tail が空っぽであったということは、 head も空っぽなはずで、
                // 今回追加しようとしている対象は 最初の要素ということが分かります。
                debug_assert_eq!(self.len(), 0);
                // `RC::clone( )` - 所有者が増えました。
                self.head = Some(Rc::clone(&node));
            }
        }

        // とりあえず 末尾側に インスタンスのオリジナルをセットします。
        self.tail = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // tail を空っぽにします。中身は消えていないので、中身の所有者は増減しません。
        match self.tail.take() {
            Some(tail) => {
                // さっき tail に入っていたものが、 tail です。
                if let Some(prev) = tail.borrow_mut().prev.take() {
                    // tailのnextのリンクは切断します。
                    prev.borrow_mut().next = None;
                    // リンクトリストの末尾は tail になります。
                    self.tail = Some(prev);
                } else {
                    // tailのprev が空っぽであったということは、この tail が最後の１個だったわけで、
                    // リンクトリストの head も切断します。
                    debug_assert_eq!(self.len(), 1);
                    self.head = None;
                }
                self.length -= 1;
                let v = Rc::try_unwrap(tail) // Rc<RefCell<Node<T>> -> RefCell<Node<T>>
                    .ok() // Result<RefCell<Node<T>>, Rc<RefCell<Node<T>>>> -> Option<RefCell<Node<T>>>
                    .expect("Failed to Rc::try_unwrap tail node") // RefCell<Node<T>>
                    .into_inner() // RefCell<Node<T>> -> Node<T>
                    .value;
                Some(v)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        // イテレーションする都度、Iterインスタンスを作成します。
        Iter {
            // カレントを設定します。
            current: if self.len() == 0 {
                // 空っぽですから、カレントはありません。
                None
            } else {
                // &Option<T> を Option<&T> に変更し、 &T を Someでラッピングします。
                Some(Rc::clone(&self.head.as_ref().unwrap()))
            },
        }
    }
}

impl<T: fmt::Display + Clone> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let iter = self.iter();
        write!(f, "{{ head")?;
        for v in iter {
            write!(f, " -> {}", v)?;
        }
        write!(f, " }}")
    }
}
