// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> windows :: core :: Result < () > { let doc = XmlDocument :: new () ? ; doc . LoadXml (h ! ("<html>hello world</html>")) ? ; let root = doc . DocumentElement () ? ; assert ! (root . NodeName () ? == "html") ; println ! ("{:?}" , root . InnerText () ?) ; Ok (()) }
};
}
