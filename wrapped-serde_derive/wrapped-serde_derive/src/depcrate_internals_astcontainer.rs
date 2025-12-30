// Generated macro for Container (struct)
macro_rules! Depcrate_internals_astContainer {
() => {
// Module: crate::internals::ast
// Provides: {"Container"}
// Dependencies: {}
# [doc = " A source data structure annotated with `#[derive(Serialize)]` and/or `#[derive(Deserialize)]`,"] # [doc = " parsed into an internal representation."] pub struct Container < 'a > { # [doc = " The struct or enum name (without generics)."] pub ident : syn :: Ident , # [doc = " Attributes on the structure, parsed for Serde."] pub attrs : attr :: Container , # [doc = " The contents of the struct or enum."] pub data : Data < 'a > , # [doc = " Any generics on the struct or enum."] pub generics : & 'a syn :: Generics , # [doc = " Original input."] pub original : & 'a syn :: DeriveInput , }
};
}
