// Generated macro for forward_to_subschema_for (macro)
macro_rules! Depcrate_serforward_to_subschema_for {
() => {
// Module: crate::ser
// Provides: {"forward_to_subschema_for"}
// Dependencies: {}
macro_rules ! forward_to_subschema_for { ($ fn : ident , $ ty : ty) => { fn $ fn (self , _value : $ ty) -> Result < Self :: Ok , Self :: Error > { Ok (self . generator . subschema_for ::<$ ty > ()) } } ; }
};
}
