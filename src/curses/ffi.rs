use std::ffi;

/*
Arrow key codes: 
KEY_UP: 259
KEY_DOWN: 258
KEY_RIGHT: 261
KEY_LEFT: 260
*/
pub mod constants {
    pub const KEY_UP: i32 = 259;
    pub const KEY_DOWN: i32 = 258;
    pub const KEY_RIGHT: i32 = 261;
    pub const KEY_LEFT: i32 = 260;
    pub const KEY_ENTER: i32 = 10;
    pub const KEY_BACKSPACE: i32 = 127;
}

#[link(name = "ncurses")]
extern "C" {
    // window stuff
    pub fn initscr();
    pub fn refresh();
    pub fn endwin();
    pub fn noecho();
    pub fn keypad(win: *mut u8, bf: libc::c_int);
    pub fn newwin(nlines: libc::c_int, ncols: libc::c_int, begin_x: libc::c_int, begin_y: libc::c_int) -> *mut u8;
    // input stuff
    pub fn getch() -> libc::c_int;
    pub fn wgetch(window: *mut u8) -> libc::c_int;
    // output stuff
    pub fn mvprintw(y: libc::c_int, x: libc::c_int, fmt: *const libc::c_char);
    pub fn mvwprintw(win: *mut u8, y: libc::c_int, x: libc::c_int, fmt: *const libc::c_char);
}