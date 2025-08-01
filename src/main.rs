use crate::{curses::{enable_keypad, init_curses}, providers::{keyboard_provider::{close_window_after_keypress, keyboard_handler}, render_provider}, state::{FluxStore, State}};

mod curses;
mod providers;
mod state;

fn state_logger(state: &State) {
    print!("{:?}\n", state);
    print!("-----------\n");
}

fn main() {
    let curses_context = init_curses(100, 100);
    enable_keypad(&curses_context);
    let state = State::new(100, 100, curses_context);
    let mut flux_store = FluxStore::new(state);

    flux_store.add_listener(render_provider::render_state);
    // flux_store.add_listener(state_logger);
    // flux_store.execute(&|state: &mut State| state.editor_state.content = vec![vec!['a']]);
    loop { // For debugging purposes 
        keyboard_handler(&mut flux_store);
    }
    // keyboard_provider::close_window_after_keypress();
}
