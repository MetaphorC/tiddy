use ncurses::*;

fn main() {
    initscr();
    addstr("Hello, World!");
    refresh();

    let mut quit = false;

    while !quit {
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }

    getch();
   
    endwin();
}
