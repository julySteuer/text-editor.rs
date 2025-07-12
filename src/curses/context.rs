use crate::curses::ffi::newwin;

#[derive(Debug)]
pub struct CursesContext {
    window: *mut u8,
}

impl CursesContext {
    pub fn new(nlines: i32, ncols: i32) -> CursesContext {
        let window_ptr = {
            unsafe {
                newwin(nlines, ncols, 0, 0)
            }
        };
        CursesContext { window: window_ptr }
    }

    pub fn get_window_ptr(&self) -> *mut u8 {
        self.window
    }
}