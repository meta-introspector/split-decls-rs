// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < S , T > (value : & T , serializer : S) -> Result < S :: Ok , S :: Error > where T : Copy + VarInt , S : Serializer , { (* value) . serialize (serializer) }
};
}
