use std::cell::RefCell;
use std::rc::Rc;

use crate::Node;

impl<T> Node<T> {
    /// 対象を持ちます。 `Rc<RefCell< >>` でラッピングするので、
    /// 所有権の所持者は複数になれます。（Reference counting）
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            prev: None,
            next: None,
        }))
    }
}
