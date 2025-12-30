// Generated macro for Component (struct)
macro_rules! DepcrateComponent {
() => {
// Module: crate
// Provides: {"Component"}
// Dependencies: {}
# [doc = " Helper structure representing a single component found in a test directory."] struct Component { # [doc = " The name of this component, inferred from the file stem."] # [doc = ""] # [doc = " May be shared across different languages."] name : String , # [doc = " The path to the source file for this component."] path : PathBuf , # [doc = " Whether or not this component is a \"runner\" or a \"test\""] kind : Kind , # [doc = " The detected language for this component."] language : Language , # [doc = " The WIT world that's being used with this component, loaded from"] # [doc = " `test.wit`."] bindgen : Bindgen , # [doc = " The contents of the test file itself."] contents : String , # [doc = " The contents of the test file itself."] lang_config : Option < HashMap < String , toml :: Value > > , }
};
}
