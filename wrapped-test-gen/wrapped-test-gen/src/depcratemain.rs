// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [doc = " Generates test files in {quick-xml}/tests/documents/encoding/{}.xml"] fn main () { let index = "encoding/indexes.json" ; let file = File :: open (index) . expect (& format ! (r#"unable to load `{}`. Probably `encoding` submodule does not fetched? Try to run

        git submodule update --init -- encoding

        in the current working dir (i. e. <quick-xml>/test-gen/)
        "# , index)) ; let indexes : Indexes = from_reader (file) . expect (& format ! ("invalid format of `{}`" , index)) ; process_index (BIG5 , & indexes . big5) ; process_index (EUC_KR , & indexes . euc_kr) ; process_index (GBK , & indexes . gb18030) ; process_index (GB18030 , & indexes . gb18030) ; process_index (EUC_JP , & indexes . jis0208) ; process_index (ISO_2022_JP , & indexes . jis0208) ; process_index (SHIFT_JIS , & indexes . jis0208) ; for (label , codepoints) in indexes . single_byte . into_iter () { let enc = Encoding :: for_label (label . as_bytes ()) . expect (& format ! ("label `{}` is unsupported" , label)) ; process_index (enc , & codepoints) ; if enc == ISO_8859_8 { process_index (ISO_8859_8_I , & codepoints) ; } } make_xml (X_USER_DEFINED , '\u{F780}' ..= '\u{F7FF}') ; }
};
}
