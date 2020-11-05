extern crate linked_list;

use linked_list::LinkedList;

fn main() {
    println!("Start.");

    let mut list: LinkedList<i32> = LinkedList::new();
    (0..10).for_each(|n| list.append(n));

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), Some(0));
    assert_eq!(iter.next_back(), None);

    println!("Finished.");
}
