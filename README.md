# Another unnecessary linked list...

---

A doubly linked list is implemented using internal `Node` and the `LinkedList` struct itself. Thread-safe shared mutability is implemented using `Arc<Mutex<T>>`.

`Node<T>` struct:
```rs
struct Node<T> {
    value: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Arc<Mutex<Node<T>>>>,
}
```

`LinkedList<T>` struct:
```rs
pub struct LinkedList<T> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
    len: u32,
}
```
---

## 👨‍💻 Author
**Rostyslav Kashper**  
Rust dev / Game dev  
GitHub: _[FantRS](https://github.com/FantRS)_
