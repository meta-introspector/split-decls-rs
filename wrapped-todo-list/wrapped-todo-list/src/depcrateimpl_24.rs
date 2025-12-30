// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl App { fn run (mut self , terminal : & mut DefaultTerminal) -> Result < () > { while ! self . should_exit { terminal . draw (| frame | frame . render_widget (& mut self , frame . area ())) ? ; if let Some (key) = event :: read () ? . as_key_press_event () { self . handle_key (key) ; } } Ok (()) } fn handle_key (& mut self , key : KeyEvent) { match key . code { KeyCode :: Char ('q') | KeyCode :: Esc => self . should_exit = true , KeyCode :: Char ('h') | KeyCode :: Left => self . select_none () , KeyCode :: Char ('j') | KeyCode :: Down => self . select_next () , KeyCode :: Char ('k') | KeyCode :: Up => self . select_previous () , KeyCode :: Char ('g') | KeyCode :: Home => self . select_first () , KeyCode :: Char ('G') | KeyCode :: End => self . select_last () , KeyCode :: Char ('l') | KeyCode :: Right | KeyCode :: Enter => { self . toggle_status () ; } _ => { } } } const fn select_none (& mut self) { self . todo_list . state . select (None) ; } fn select_next (& mut self) { self . todo_list . state . select_next () ; } fn select_previous (& mut self) { self . todo_list . state . select_previous () ; } const fn select_first (& mut self) { self . todo_list . state . select_first () ; } const fn select_last (& mut self) { self . todo_list . state . select_last () ; } # [doc = " Changes the status of the selected list item"] fn toggle_status (& mut self) { if let Some (i) = self . todo_list . state . selected () { self . todo_list . items [i] . status = match self . todo_list . items [i] . status { Status :: Completed => Status :: Todo , Status :: Todo => Status :: Completed , } } } }
};
}
