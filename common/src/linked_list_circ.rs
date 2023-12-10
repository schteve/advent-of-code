use std::{cmp::Ordering, mem};

#[derive(Clone)]
struct LlNode<T> {
    prev: usize,
    next: usize,
    value: T,
}

#[derive(Clone)]
pub struct LlIter<'a, T> {
    list: &'a LinkedListCirc<T>,
    index: Option<usize>,
}

impl<'a, T> LlIter<'a, T> {
    fn new(list: &'a LinkedListCirc<T>) -> Self {
        let index = list.head;
        Self { list, index }
    }
}

impl<'a, T> Iterator for LlIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head_idx) = self.list.head {
            if let Some(idx) = self.index {
                // Traverse to the next node, or stop if the head has been reached
                if self.list.data[idx].next == head_idx {
                    self.index = None;
                } else {
                    self.index = Some(self.list.data[idx].next);
                }

                Some(&self.list.data[idx].value)
            } else {
                // List has been completely traversed
                None
            }
        } else {
            // List is empty
            None
        }
    }
}

pub struct LlIterMut<'a, T> {
    list: &'a mut LinkedListCirc<T>,
    index: Option<usize>,
}

impl<'a, T> LlIterMut<'a, T> {
    fn new(list: &'a mut LinkedListCirc<T>) -> Self {
        let index = list.head;
        Self { list, index }
    }
}

impl<'a, T> Iterator for LlIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head_idx) = self.list.head {
            if let Some(idx) = self.index {
                // Traverse to the next node, or stop if the head has been reached
                if self.list.data[idx].next == head_idx {
                    self.index = None;
                } else {
                    self.index = Some(self.list.data[idx].next);
                }

                let ptr = &mut self.list.data[idx].value as *mut T;
                unsafe { Some(&mut *ptr) }
            } else {
                // List has been completely traversed
                None
            }
        } else {
            // List is empty
            None
        }
    }
}

#[derive(Clone)]
pub struct LinkedListCirc<T> {
    data: Vec<LlNode<T>>,
    head: Option<usize>,
    free_list: Option<usize>,
    last_op_idx: Option<usize>,
}

impl<T: Default> LinkedListCirc<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            head: None,
            free_list: None,
            last_op_idx: None,
        }
    }

    pub fn len(&self) -> usize {
        // Should len be maintained as a member of the struct? Could be used directly for a few things elsewhere.
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn allocate_node(&mut self) -> usize {
        // If there are any free indexes, use those. Otherwise create a new index.
        if let Some(free_idx) = self.free_list {
            if self.data[free_idx].next == free_idx {
                // There is only one node in the list
                self.free_list = None;
            } else {
                let before_idx = self.data[free_idx].prev;
                let after_idx = self.data[free_idx].next;
                self.data[before_idx].next = after_idx;
                self.data[after_idx].prev = before_idx;

                // Free list should now start with the next node
                self.free_list = Some(self.data[free_idx].next);
            }
            free_idx
        } else {
            let new_node_idx = self.data.len();
            let new_node = LlNode {
                // Dummy values
                prev: 0,
                next: 0,
                value: T::default(),
            };
            self.data.push(new_node);
            new_node_idx
        }
    }

    fn free_node(&mut self, index: usize) {
        // If there is an existing free list, append to it. Otherwise create a new one.
        if let Some(free_idx) = self.free_list {
            let before_idx = self.data[free_idx].prev;
            let after_idx = free_idx;
            self.data[before_idx].next = index;
            self.data[after_idx].prev = index;
            self.data[index].prev = before_idx;
            self.data[index].next = after_idx;
        } else {
            self.free_list = Some(index);
            self.data[index].prev = index;
            self.data[index].next = index;
        }
    }

    fn get_offset_from_idx(&self, start: usize, offset: i32) -> usize {
        let mut idx = start;
        match offset.cmp(&0) {
            Ordering::Greater => {
                for _ in 0..offset {
                    idx = self.data[idx].next;
                }
            }
            Ordering::Less => {
                for _ in offset..0 {
                    idx = self.data[idx].prev;
                }
            }
            Ordering::Equal => (),
        }
        idx
    }

    // NOTE: the input is a raw index so you better know that it's on the list!
    fn insert_at_idx(&mut self, value: T, idx: Option<usize>) {
        // Special case - list is empty
        if self.head.is_none() {
            let new_node_idx = self.allocate_node();

            self.data[new_node_idx].prev = new_node_idx;
            self.data[new_node_idx].next = new_node_idx;
            self.data[new_node_idx].value = value;

            self.head = Some(new_node_idx);
            self.last_op_idx = Some(new_node_idx);
        } else {
            // Move to the target node's index - here we use the "after" node as the target
            let target_idx = idx.unwrap();

            // Create the new node, then fixup the previous and next nodes
            let before_idx = self.data[target_idx].prev;
            let after_idx = target_idx;
            let new_node_idx = self.allocate_node();

            self.data[new_node_idx].prev = before_idx;
            self.data[new_node_idx].next = after_idx;
            self.data[new_node_idx].value = value;

            self.data[before_idx].next = new_node_idx;
            self.data[after_idx].prev = new_node_idx;

            self.last_op_idx = Some(new_node_idx);
        }
    }

    pub fn insert(&mut self, value: T, offset: i32) {
        let target_idx = self.head.map(|idx| self.get_offset_from_idx(idx, offset));
        self.insert_at_idx(value, target_idx);
    }

    pub fn insert_from_last_op(&mut self, value: T, offset: i32) {
        let target_idx = self
            .last_op_idx
            .map(|idx| self.get_offset_from_idx(idx, offset));
        self.insert_at_idx(value, target_idx);
    }

    // NOTE: the input is a raw index so you better know that it's on the list!
    fn remove_at_idx(&mut self, idx: usize) -> T {
        // Adjust head (if needed) before modifying the list
        let head_idx = self.head.expect("Tried to remove from empty list");
        if head_idx == idx {
            // Only need to adjust head if we are removing the head node
            if self.data[head_idx].next == head_idx {
                // List only had one node
                self.head = None;
            } else {
                // List had multiple nodes
                self.head = Some(self.data[head_idx].next);
            }
        }

        // Free the node, then fixup the previous and next nodes
        let before_idx = self.data[idx].prev;
        let after_idx = self.data[idx].next;

        let existing_value = mem::take(&mut self.data[idx].value);
        self.free_node(idx);

        // If we are removing the last node
        if self.head.is_none() {
            self.last_op_idx = None;
        } else {
            self.data[before_idx].next = after_idx;
            self.data[after_idx].prev = before_idx;

            self.last_op_idx = Some(after_idx);
        }

        existing_value
    }

    pub fn remove(&mut self, offset: i32) -> T {
        let target_idx = self
            .head
            .map(|idx| self.get_offset_from_idx(idx, offset))
            .expect("Tried to remove from empty list");
        self.remove_at_idx(target_idx)
    }

    pub fn remove_from_last_op(&mut self, offset: i32) -> T {
        let target_idx = self
            .last_op_idx
            .map(|idx| self.get_offset_from_idx(idx, offset))
            .expect("Last op must exist");
        self.remove_at_idx(target_idx)
    }

    pub fn iter(&self) -> LlIter<T> {
        LlIter::new(self)
    }

    pub fn iter_mut(&mut self) -> LlIterMut<T> {
        LlIterMut::new(self)
    }
}

impl<T: Clone + Default> LinkedListCirc<T> {
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

impl<T: Default> Default for LinkedListCirc<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nodes() {
        let mut list: LinkedListCirc<u32> = LinkedListCirc::new();
        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 0);
        assert_eq!(list.free_list, None);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 1);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 2);

        let node_idx = list.allocate_node();
        assert_eq!(node_idx, 3);

        list.free_node(0);
        assert_eq!(list.free_list, Some(0));

        list.free_node(2);
        assert_eq!(list.free_list, Some(0));
        assert_eq!(list.data[0].next, 2);
        assert_eq!(list.data[2].next, 0);
    }

    #[test]
    fn test_insert_remove() {
        let mut list = LinkedListCirc::new();
        list.insert(0, 0);
        assert_eq!(list.head, Some(0));

        list.remove(0);
        assert_eq!(list.head, None);

        for i in 0..10 {
            list.insert(i, 0);
        }
        assert_eq!(list.to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut list = LinkedListCirc::new();
        list.insert(0, 0);
        list.insert(1, 0);
        assert_eq!(list.to_vec(), vec![0, 1]);
        assert_eq!(list.head, Some(0));

        list.insert(2, 1);
        assert_eq!(list.to_vec(), vec![0, 2, 1]);

        list.insert(3, 0);
        assert_eq!(list.to_vec(), vec![0, 2, 1, 3]);

        list.insert(4, -11);
        assert_eq!(list.to_vec(), vec![0, 4, 2, 1, 3]);

        let item = list.remove(2);
        assert_eq!(item, 2);
        assert_eq!(list.to_vec(), vec![0, 4, 1, 3]);
        assert_eq!(list.head, Some(0));

        let item = list.remove(0);
        assert_eq!(item, 0);
        assert_eq!(list.to_vec(), vec![4, 1, 3]);
        assert_eq!(list.head, Some(4));

        let item = list.remove(1);
        assert_eq!(item, 1);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));

        list.insert(5, 2);
        assert_eq!(list.to_vec(), vec![4, 3, 5]);
        assert_eq!(list.head, Some(4));

        let item = list.remove(2);
        assert_eq!(item, 5);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));

        let item = list.remove(7);
        assert_eq!(item, 3);
        assert_eq!(list.to_vec(), vec![4]);
        assert_eq!(list.head, Some(4));

        let item = list.remove(0);
        assert_eq!(item, 4);
        assert_eq!(list.to_vec(), vec![]);
        assert_eq!(list.head, None);
    }

    #[test]
    fn test_insert_remove_from_last_op() {
        let mut list = LinkedListCirc::new();
        list.insert_from_last_op(0, 0);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.last_op_idx, Some(0));

        list.remove_from_last_op(0);
        assert_eq!(list.head, None);
        assert_eq!(list.last_op_idx, None);

        for i in 0..10 {
            list.insert_from_last_op(i, 1);
        }
        assert_eq!(list.to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut list = LinkedListCirc::new();
        list.insert_from_last_op(0, 0);
        list.insert_from_last_op(1, 0);
        assert_eq!(list.to_vec(), vec![0, 1]);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.last_op_idx, Some(1));

        list.insert_from_last_op(2, 0);
        assert_eq!(list.to_vec(), vec![0, 2, 1]);
        assert_eq!(list.last_op_idx, Some(2));

        list.insert_from_last_op(3, -1);
        assert_eq!(list.to_vec(), vec![0, 2, 1, 3]);
        assert_eq!(list.last_op_idx, Some(3));

        list.insert_from_last_op(4, -10);
        assert_eq!(list.to_vec(), vec![0, 4, 2, 1, 3]);
        assert_eq!(list.last_op_idx, Some(4));

        let item = list.remove_from_last_op(1);
        assert_eq!(item, 2);
        assert_eq!(list.to_vec(), vec![0, 4, 1, 3]);
        assert_eq!(list.head, Some(0));
        assert_eq!(list.last_op_idx, Some(1));

        let item = list.remove_from_last_op(-2);
        assert_eq!(item, 0);
        assert_eq!(list.to_vec(), vec![4, 1, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.last_op_idx, Some(4));

        let item = list.remove_from_last_op(1);
        assert_eq!(item, 1);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.last_op_idx, Some(3));

        list.insert_from_last_op(5, 1);
        assert_eq!(list.to_vec(), vec![4, 3, 5]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.last_op_idx, Some(2)); // Node indexes should be reallocated in the order they were removed

        let item = list.remove_from_last_op(0);
        assert_eq!(item, 5);
        assert_eq!(list.to_vec(), vec![4, 3]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.last_op_idx, Some(4));

        let item = list.remove_from_last_op(7);
        assert_eq!(item, 3);
        assert_eq!(list.to_vec(), vec![4]);
        assert_eq!(list.head, Some(4));
        assert_eq!(list.last_op_idx, Some(4));

        let item = list.remove_from_last_op(0);
        assert_eq!(item, 4);
        assert_eq!(list.to_vec(), vec![]);
        assert_eq!(list.head, None);
        assert_eq!(list.last_op_idx, None);
    }
}
