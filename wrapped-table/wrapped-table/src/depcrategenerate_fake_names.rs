// Generated macro for generate_fake_names (function)
macro_rules! Depcrategenerate_fake_names {
() => {
// Module: crate
// Provides: {"generate_fake_names"}
// Dependencies: {}
fn generate_fake_names () -> Vec < Data > { use fakeit :: { address , contact , name } ; (0 .. 20) . map (| _ | { let name = name :: full () ; let address = format ! ("{}\n{}, {} {}" , address :: street () , address :: city () , address :: state () , address :: zip ()) ; let email = contact :: email () ; Data { name , address , email , } }) . sorted_by (| a , b | a . name . cmp (& b . name)) . collect () }
};
}
