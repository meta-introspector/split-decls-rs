// Generated macro for default_on_eof (function)
macro_rules! Depcratedefault_on_eof {
() => {
// Module: crate
// Provides: {"default_on_eof"}
// Dependencies: {}
# [doc = " This helper function enables successful deserialization of versioned structs; new structs may"] # [doc = " include additional fields if they impl Default and are added to the end of the struct. Right"] # [doc = " now, this function is targeted at `bincode` deserialization; the error match may need to be"] # [doc = " updated if another package needs to be used in the future."] pub fn default_on_eof < 'de , T , D > (d : D) -> Result < T , D :: Error > where D : Deserializer < 'de > , T : Deserialize < 'de > + Default , { let result = T :: deserialize (d) ; ignore_eof_error :: < 'de , T , D :: Error > (result) }
};
}
