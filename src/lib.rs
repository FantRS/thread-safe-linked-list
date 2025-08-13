pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

    #[test]
    fn push_back_test() {
        let mut list: LinkedList<i32> = LinkedList::default();
        _ = list.push_back(10);
        _ = list.push_back(50);

        assert_eq!(list.get_tail().unwrap(), 50);
    }

    #[test]
    fn push_front_test() {
        let mut list: LinkedList<i32> = LinkedList::default();
        _ = list.push_back(10);
        _ = list.push_front(50);

        assert_eq!(list.get_head().unwrap(), 50);
    }

    #[test]
    fn pop_back_test() {
        let mut list: LinkedList<i32> = LinkedList::default();
        _ = list.push_back(10);
        _ = list.push_back(50);
        _ = list.pop_back();

        assert_eq!(list.get_tail().unwrap(), 10);
    }

    #[test]
    fn pop_front_test() {
        let mut list: LinkedList<i32> = LinkedList::default();
        _ = list.push_back(10);
        _ = list.push_back(50);
        _ = list.pop_front();

        assert_eq!(list.get_tail().unwrap(), 50);
    }
}
