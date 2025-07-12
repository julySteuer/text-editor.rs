use std::ffi::CString;

use crate::curses::{context::CursesContext, ffi::refresh};

pub mod ffi;
pub mod context;

pub fn init_curses(width: i32, height: i32) -> CursesContext {
    unsafe {
        ffi::initscr();       
    }
    CursesContext::new(width, height)
}

pub fn enable_keypad(context: &CursesContext) {
    unsafe {
        ffi::noecho();
        ffi::keypad(context.get_window_ptr(), 1);
    }
}

pub fn mvprint_str(y: i32, x: i32, str: &str) {
    let inp_str = CString::new(str).expect("CString could not be created");
    unsafe {
        ffi::mvprintw(y, x, inp_str.as_ptr());
    }
}

pub fn mvprintw_str(w: *mut u8, y: i32, x: i32, str: &str) {
    let inp_str = CString::new(str).expect("CString could not be created");
    unsafe {
        ffi::mvwprintw(w, y, x, inp_str.as_ptr());
    }
}

pub fn wgetc(window: *mut u8) -> i32 {
    unsafe {
        return ffi::wgetch(window);
    }
}