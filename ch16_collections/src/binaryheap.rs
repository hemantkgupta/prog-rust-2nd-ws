use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

pub fn binaryheap_work(){

    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

    if let Some(top) = heap.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);
        }
    }

    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));

}