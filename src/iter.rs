use crate::Iter;
use std::rc::Rc;

/// 前へ進むイテレーターです。
impl<T: Clone> Iterator for Iter<T> {
    type Item = T;
    /// カレントを返します。
    fn next(&mut self) -> Option<Self::Item> {
        // カレントを空にし、中身を取り出します。
        match self.current.take() {
            // 中身はありませんでした。イテレーターは終わります。
            None => None,
            Some(curr) => {
                // 中身を借り物（変更不可能を明示）にします。クローン可能になります。
                let curr = curr.borrow();
                // カレントをコピーします。
                let v = curr.value.clone();
                // ネクストの有無。
                match curr.next {
                    None => {
                        // カレントを無くします。イテレーターは終わります。
                        self.current = None;
                    }
                    Some(ref next) => {
                        // ネクストは、次からカレントになります。
                        self.current = Some(Rc::clone(next));
                    }
                }
                Some(v)
            }
        }
    }
}

/// 後ろへ進むイテレーターです。
impl<T: Clone> DoubleEndedIterator for Iter<T> {
    /// プレビアスを返します。
    fn next_back(&mut self) -> Option<T> {
        // カレントを空にし、中身を取り出します。
        match self.current.take() {
            // 中身はありませんでした。イテレーターは終わります。
            None => None,
            Some(curr) => {
                // 中身を借り物（変更不可能を明示）にします。クローン可能になります。
                let curr = curr.borrow();
                // プレビアスの有無。
                match curr.prev {
                    None => {
                        // カレントを無くします。イテレーターは終わります。
                        self.current = None;
                        None
                    }
                    Some(ref prev) => {
                        // プレビアスは、次からカレントになります。
                        self.current = Some(Rc::clone(prev));
                        Some(prev.borrow().value.clone())
                    }
                }
            }
        }
    }
}
