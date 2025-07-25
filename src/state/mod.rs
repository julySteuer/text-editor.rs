use crate::curses::context::CursesContext;

#[derive(Debug)]
pub struct ApplicationState {
    pub context: CursesContext,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct CursorState {
    pub x: i32,
    pub y: i32,
    pub desired_x: i32,
    pub desired_y: i32
}

/*
Could use Ropes to make it faster.
Doubly linked list with LRU Cache could also improve performance 
*/
#[derive(Debug)]
pub struct EditorState {
    pub cursor: CursorState,
    pub content: Vec<Vec<char>>
}

#[derive(Debug)]
pub struct State {
    pub application_state: ApplicationState,
    pub editor_state: EditorState
}

pub struct FluxStore {
    listeners: Vec<fn(&State)>,
    state: State
}

impl FluxStore {
    pub fn new(state: State) -> FluxStore {
        FluxStore { listeners: Vec::new(), state: state }
    }

    pub fn add_listener(&mut self, listener: fn(&State)) {
        self.listeners.push(listener);
    }

    pub fn execute(&mut self, action: &dyn Fn(&mut State)) {
        action(&mut self.state);
        self.notify();
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    fn notify(&self) {
        self.listeners.iter().for_each(|f| {
            f(&self.state);
        });
    }
}

impl State {
    pub fn new(width: i32, height: i32, curses_ctx: CursesContext) -> State {
        let application_state = ApplicationState { context: curses_ctx, width: width, height: height };
        let editor_state = EditorState { content: vec![vec![]], cursor: CursorState { x: 0, y: 0, desired_x: 0, desired_y: 0 } }; // Empty 2d vec so one starts on first line 
        State { application_state, editor_state }
    }
}

pub mod helper {
    use crate::state::CursorState;

    pub fn update_cursor_and_desired_cursor(state: &mut CursorState, x: i32, y: i32) {
        state.desired_x = x;
        state.x = x;
        state.y = y;
    }
}
