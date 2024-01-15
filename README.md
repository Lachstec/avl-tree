# avl-tree
An unsafe implementation of avl trees in rust, based mostly on this great [article](https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/) by Francis Murillo.

## About
The avl tree implemented here does only serve educative purposes and **should not** be used in production. The purpose of this project is to implement and visualize an avl tree in a non-garbage collected and (probably) safe language. As far as I was willing to test, Miri seems to be happy, but that is no guarantee that this wont blow up in your face. I am very inexperienced in writing unsafe rust, so be very cautious.

## Getting started
Currently, the only meaningful thing that you can do is run some unit tests by using 
```bash
cargo test
```
It is planned to add support for visualizing the resulting tree in order to better understand how it works.