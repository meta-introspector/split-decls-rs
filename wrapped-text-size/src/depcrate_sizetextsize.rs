// Generated macro for TextSize (struct)
macro_rules! Depcrate_sizeTextSize {
() => {
// Module: crate::size
// Provides: {"TextSize"}
// Dependencies: {}
# [doc = " A measure of text length. Also, equivalently, an index into text."] # [doc = ""] # [doc = " This is a UTF-8 bytes offset stored as `u32`, but"] # [doc = " most clients should treat it as an opaque measure."] # [doc = ""] # [doc = " For cases that need to escape `TextSize` and return to working directly"] # [doc = " with primitive integers, `TextSize` can be converted losslessly to/from"] # [doc = " `u32` via [`From`] conversions as well as losslessly be converted [`Into`]"] # [doc = " `usize`. The `usize -> TextSize` direction can be done via [`TryFrom`]."] # [doc = ""] # [doc = " These escape hatches are primarily required for unit testing and when"] # [doc = " converting from UTF-8 size to another coordinate space, such as UTF-16."] # [derive (Clone , Copy , Default , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct TextSize { pub (crate) raw : u32 , }
};
}
