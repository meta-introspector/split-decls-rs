// Generated macro for weekdays (module)
macro_rules! Depcrate_datetime_legacyweekdays {
() => {
// Module: crate::datetime::legacy
// Provides: {"weekdays"}
// Dependencies: {}
# [doc = "Formatting symbols for [`Weekday`](crate::provider::fields::FieldSymbol::Weekday)."] # [doc = ""] # [doc = "For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol)."] pub mod weekdays { use super :: * ; # [doc = "Locale data for Weekday corresponding to the symbols."] pub struct Symbols < 'data > (pub [Cow < 'data , str > ; 7]) ; # [doc = "Symbol data for the \"format\" style formatting of Weekday."] # [doc = ""] # [doc = "The format style is used in contexts where it is different from the stand-alone form, ex: a case inflected form where the stand-alone form is the nominative case."] pub struct FormatWidths < 'data > { # [doc = "Short length symbol for \"format\" style symbol for weekdays, if present."] pub short : Option < Symbols < 'data > > , } pub struct Contexts < 'data > { # [doc = " The symbol data for \"format\" style symbols."] pub format : FormatWidths < 'data > , # [doc = " Whether or not there are \"standalone\" style symbols"] pub stand_alone : bool , } }
};
}
