# tiddy
a vim spirited terminal todo app written in rust | forked from [todo-rs by Tsoding](https://github.com/tsoding/todo-rs)

## setup
*to build, and run, make sure you have rust and cargo installed..*

..if not then you can install it from this page in the rust book: [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

```console 
git clone https://github.com/metaphorc/tiddy

cd tiddy

cargo build
```
## Manual

|Keybinds|Effect|
|---|---|
|<kbd>k</kbd>, <kbd>j</kbd>|Move up and down|
|<kbd>Shift+K</kbd>, <kbd>Shift+J</kbd>|Move selected item up or down|
|<kbd>g</kbd>, <kbd>G</kbd> | Jump to the start, and end of the current item list|
|<kbd>i</kbd>|Insert a new element|
|<kbd>r</kbd>|Rename the current element|
|<kbd>d</kbd>|Delete the current element|
|<kbd>q</kbd>|Quit and save state|
|<kbd>TAB</kbd>|Switch panels|
|<kbd>Enter</kbd>|Move element to and fro between TODO and DONE|
