// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame , temperatures : & [u8]) { let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Fill (1)]) . spacing (1) ; let [title , main] = frame . area () . layout (& layout) ; frame . render_widget ("Weather demo" . bold () . into_centered_line () , title) ; frame . render_widget (vertical_barchart (temperatures) , main) ; }
};
}
