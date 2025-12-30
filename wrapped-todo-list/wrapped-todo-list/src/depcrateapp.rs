// Generated macro for App (struct)
macro_rules! DepcrateApp {
() => {
// Module: crate
// Provides: {"App"}
// Dependencies: {}
# [doc = " This struct holds the current state of the app. In particular, it has the `todo_list` field"] # [doc = " which is a wrapper around `ListState`. Keeping track of the state lets us render the"] # [doc = " associated widget with its state and have access to features such as natural scrolling."] # [doc = ""] # [doc = " Check the event handling at the bottom to see how to change the state on incoming events. Check"] # [doc = " the drawing logic for items on how to specify the highlighting style for selected items."] struct App { should_exit : bool , todo_list : TodoList , }
};
}
