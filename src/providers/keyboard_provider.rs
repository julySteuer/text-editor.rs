use crate::{curses::{self, ffi::constants::{KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP}, wgetc}, state::{FluxStore, State}};

pub fn close_window_after_keypress() {
    unsafe {
        curses::ffi::getch();
        curses::ffi::endwin();
    }
}

// TODO: Find better way for cursor state alias (maybe just giving the reference to the cursor state))

fn handle_standard_key(keycode: i32, flux_store: &mut FluxStore) {
    let c = char::from_u32(keycode as u32).expect("Value could not be converted");

    flux_store.execute(&move |state: &mut State| {
        let row = state.editor_state.content.get_mut(state.editor_state.cursor.y as usize);
        row.map(|elem| {
            elem.insert(state.editor_state.cursor.x as usize, c);
        });
        update_cursor_and_desired_cursor(state, state.editor_state.cursor.x+1, state.editor_state.cursor.y);
    });
}

fn handle_enter(flux_store: &mut FluxStore) {
    flux_store.execute(&|state: &mut State| {
        update_cursor_and_desired_cursor(state, 0, state.editor_state.cursor.y + 1);
        state.editor_state.content.insert(state.editor_state.cursor.y as usize, Vec::new());
    });
}

fn handle_arrow_key(keycode: i32, flux_store: &mut FluxStore) {
    let relative_move = {
        match keycode {
            KEY_DOWN => (0, 1),
            KEY_UP => (0, -1),
            KEY_RIGHT => (1, 0),
            KEY_LEFT => (-1, 0),
            _ => panic!("Internal Error")
        }
    };

    // TODO: Bound check 

    flux_store.execute(&|state: &mut State| {
        let cursor_state = &mut state.editor_state.cursor;
        let new_line_length = state.editor_state.content.get((cursor_state.y + relative_move.1) as usize).unwrap().len();
        cursor_state.desired_x += relative_move.0;
        if new_line_length < cursor_state.desired_x as usize {
            cursor_state.x = new_line_length as i32;
        } else {
            cursor_state.x = cursor_state.desired_x;
        }
        state.editor_state.cursor.y += relative_move.1; 
    });
}

fn update_cursor_and_desired_cursor(state: &mut State, x: i32, y: i32) {
    state.editor_state.cursor.desired_x = x;
    state.editor_state.cursor.x = x;
    state.editor_state.cursor.y = y;
}

pub fn keyboard_handler(flux_store: &mut FluxStore) {
    let current_state = flux_store.get_state();
    let event = wgetc(current_state.application_state.context.get_window_ptr());
    match event {
        KEY_DOWN | KEY_UP | KEY_RIGHT | KEY_LEFT => handle_arrow_key(event, flux_store),
        KEY_ENTER => handle_enter(flux_store),
        _ => handle_standard_key(event, flux_store), 
    }
}