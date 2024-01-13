# avl-tree
An unsafe implementation of avl trees in rust, based mostly on this great [article](https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/) by Francis Murillo.

## About
The avl tree implemented here does only serve educative purposes and should not be used in production. In fact, it currently exhibits undefined behaviour regarding stacked borrows according to miri. The purpose of this project is to implement and visualize an avl tree in a non-garbage collected and (probably) safe language.

## Getting started
Currently, the only meaningful thing that you can do is run some unit tests by using 
```bash
cargo test
```
It is planned to add support for visualizing the resulting tree in order to better understand how it works.