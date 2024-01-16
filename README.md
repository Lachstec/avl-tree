# avl-tree
An unsafe implementation of avl trees in rust, based mostly on this great [article](https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/) by Francis Murillo.

## About
The avl tree implemented here does only serve educative purposes and **should not** be used in production. The purpose of this project is to implement and visualize an avl tree in a non-garbage collected and (probably) safe language. As far as I was willing to test, Miri seems to be happy, but that is no guarantee that this wont blow up in your face. I am very inexperienced in writing unsafe rust, so be very cautious.

## Getting started
A simple CLI is provided, which allows to specify the values that should go into the tree, the output file format and if intermediate trees should also be generated. For further information, refer to the help text (also available via `--help`):
```
Program to visualize AVL-Trees

Usage: avl_tree [OPTIONS] -t <FILETYPE>

Options:
  -i                         Print intermediate Trees. This generates a file for every value in the tree
  -o <OUTPUT_DIRECTORY>      Output directory. Defaults to current working directory
  -v [<VALUES>...]           Values to put into the Tree
  -t <FILETYPE>              Whether to Output the Tree as SVGs or dotfiles [possible values: svg, dotfile, pdf]
  -h, --help                 Print help
```