# Rusty Linked List
An implementation of Linked List in Rust. This code utilises `unsafe` Rust to explore how raw pointers can be utilised in Rust to get the job done, while being cognizant of preventing memory leaks.

## How to check memory leaks?
* Install `valgrind` on your linux environement. For eg. on Debian based distros `sudo apt install valgrind`
* Run `valgrind ./target/debug/linkedlist` from the root directory of this repository

# Acknowledgement
This code can be seen as a training excercise supervised and guided by my friend **Mayur** [<https://github.com/ms747>] without whom I wouldn't have been exposed to the lovely world of Rust. Do check his profile out, you'll find some VERY interesting projects there :smiley:
