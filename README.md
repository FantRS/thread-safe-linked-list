# Another unnecessary linked list...

---

A doubly linked list is implemented using internal `Node` and the `LinkedList` struct itself. Thread-safe shared mutability is implemented using `Arc<Mutex<T>>`.

```rs
type ThreadSafeNode<T> = Arc<Mutex<Node<T>>>;

struct Node<T> {
    value: T,
    next: Option<ThreadSafeNode<T>>,
    prev: Option<ThreadSafeNode<T>>,
}

pub struct LinkedList<T> {
    head: Option<ThreadSafeNode<T>>,
    tail: Option<ThreadSafeNode<T>>,
    len: u32,
}
```
---

## 👨‍💻 Author
**Rostyslav Kashper**  
Rust dev / Game dev  
GitHub: _[FantRS](https://github.com/FantRS)_
