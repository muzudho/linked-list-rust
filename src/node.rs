use crate::Link;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub prev: Option<Link<T>>,
    pub next: Option<Link<T>>,
}

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
