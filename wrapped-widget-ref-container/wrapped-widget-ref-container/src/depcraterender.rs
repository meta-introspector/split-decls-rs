// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame) { let container = StackContainer { direction : Direction :: Vertical , widgets : vec ! [(Box :: new (& Greeting) , Constraint :: Percentage (50)) , (Box :: new (& Farewell) , Constraint :: Percentage (50)) ,] , } ; frame . render_widget (& container , frame . area ()) ; }
};
}
