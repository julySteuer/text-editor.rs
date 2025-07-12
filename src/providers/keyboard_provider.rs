use crate::{curses::{self, ffi::constants::{KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP}, wgetc}, state::{FluxStore, State}};

/*
Arrow key codes: 
KEY_UP: 259
KEY_DOWN: 258
KEY_RIGHT: 261
KEY_LEFT: 260
*/
pub fn close_window_after_keypress() {
    unsafe {
        curses::ffi::getch();
        curses::ffi::endwin();
    }
}

fn handle_standart_key(keycode: i32, flux_store: &mut FluxStore) {
    let c = char::from_u32(keycode as u32).expect("Value could not be converted");

    flux_store.execute(&move |state: &mut State| {
        let cursor = &mut state.editor_state.cursor;
        let row = state.editor_state.content.get_mut(cursor.y as usize);
        row.map(|elem| {
            elem.insert(cursor.x as usize, c);
        });
    });
}

fn handle_enter(flux_store: &mut FluxStore) {
    flux_store.execute(&|state: &mut State| {
        let cursor = &mut state.editor_state.cursor;
        cursor.y += 1;
        if cursor.y as usize >= state.editor_state.content.len() {
            state.editor_state.content.push(Vec::new());
            return;
        }
        state.editor_state.content.insert(cursor.y as usize, Vec::new());
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

    flux_store.execute(&|state: &mut State| {
       state.editor_state.cursor.x += relative_move.0;
       state.editor_state.cursor.y += relative_move.1; 
    });
}

pub fn keyboard_handler(flux_store: &mut FluxStore) {
    let current_state = flux_store.get_state();
    let event = wgetc(current_state.application_state.context.get_window_ptr());
    match event {
        KEY_DOWN | KEY_UP | KEY_RIGHT | KEY_LEFT => handle_arrow_key(event, flux_store),
        KEY_ENTER => handle_enter(flux_store),
        _ => handle_standart_key(event, flux_store), 
    }
}