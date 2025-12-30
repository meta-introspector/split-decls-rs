// Generated macro for tests (module)
macro_rules! Depcrate_terminaltests {
() => {
// Module: crate::terminal
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use tui :: backend :: TestBackend ; # [derive (Default , Clone)] struct ComplexProps { x : usize , y : String , } # [derive (Default)] struct StatefulComponent { x : usize , } # [derive (Default)] struct StatelessComponent ; impl ToplevelComponent for StatefulComponent { type Props = usize ; fn render (& mut self , props : impl Borrow < Self :: Props > , _area : Rect , _buf : & mut Buffer) { self . x += * props . borrow () ; } } impl ToplevelComponent for StatelessComponent { type Props = ComplexProps ; fn render (& mut self , _props : impl Borrow < Self :: Props > , _area : Rect , _buf : & mut Buffer) { } } # [test] fn it_does_render_with_simple_and_complex_props () { let mut term = Terminal :: new (TestBackend :: new (20 , 20)) . unwrap () ; let mut c = StatefulComponent :: default () ; term . render (& mut c , 3usize) . ok () ; assert_eq ! (c . x , 3) ; let mut c = StatelessComponent :: default () ; term . render (& mut c , ComplexProps :: default ()) . ok () ; } }
};
}
