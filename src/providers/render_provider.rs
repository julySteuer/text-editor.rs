use crate::{curses::{context::CursesContext, ffi::{refresh}, mvprintw_str}, state::{CursorState, State}};

fn render_content(curses_context: &CursesContext, state: &State) {
    let content_string: String = state.editor_state.content.iter().map(|node| 
        node.into_iter().collect::<String>() + "\n"
    ).collect();
    mvprintw_str(curses_context.get_window_ptr(), 0, 0, &content_string);
}

fn render_cursor(curses_context: &CursesContext, cursor_state: &CursorState) {
    mvprintw_str(curses_context.get_window_ptr(), cursor_state.y, cursor_state.x, "#");
}

pub fn render_state(state: &State) {
    let curses_context = &state.application_state.context;
    render_content(curses_context, state);
    render_cursor(curses_context, &state.editor_state.cursor);
    unsafe {
        refresh();
    }
}