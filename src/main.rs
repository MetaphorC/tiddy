use termion::{color, style, terminal_size};

use std::io;

fn main() {
   println!("{:?}", terminal_size()); 

    //print!("{}{}Stuff",
    //        termion::clear::All,
    //        termion::cursor::Goto(1, 1));

    //println!("{}Red", color::Fg(color::Red));
    //println!("{}Blue", color::Fg(color::Blue));
    //println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    //println!("{}Just plain italic", style::Italic);
}
