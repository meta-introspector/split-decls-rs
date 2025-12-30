// Generated macro for stress (macro)
macro_rules! Depcratestress {
() => {
// Module: crate
// Provides: {"stress"}
// Dependencies: {}
macro_rules ! stress { (type $ name : ident : $ ($ i : expr ,) *) => { type $ name : $ (From < [() ; $ i] > + Into < [() ; $ i] > +) *; } ; (fn $ ($ underscore : tt) *) => { const _ : () = { $ ({ fn _f < T : Trait > () { # [derive (Copy , Clone)] struct Foo < X > (X) ; let $ underscore = Foo (T :: X :: default ()) . clone () ; } }) * } ; } ; }
};
}
