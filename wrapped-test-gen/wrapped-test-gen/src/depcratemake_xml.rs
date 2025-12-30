// Generated macro for make_xml (function)
macro_rules! Depcratemake_xml {
() => {
// Module: crate
// Provides: {"make_xml"}
// Dependencies: {}
fn make_xml < I > (enc : & 'static Encoding , codepoints : I) where I : IntoIterator < Item = char > , { println ! ("{} - single:{}, ascii:{}" , enc . name () , enc . is_single_byte () , enc . is_ascii_compatible ()) ; println ! ("  - making alphabet") ; let alphabet = make_alphabet (enc , codepoints) ; println ! ("  - making xml") ; let name = alphabet . replace (| ch | ! is_xml11_name_start_char (ch) , "") ; let xml = format ! (r#"<?xml version="1.1" encoding="{encoding}"?>
<!--This is generated file. Edit <quick-xml>/test-gen/src/main.rs instead-->
<root attribute1="{attr1}"
      attribute2='{attr2}'
      {attr_name}={attr3}
>
  <?{pi}?>
  <!--{comment}-->
  {text}
  <ns:{element} ns:attribute="value1" xmlns:ns="namespace"/>
  <![CDATA[{text}]]>
</root>"# , encoding = enc . name () , attr1 = alphabet . replace (| ch | matches ! (ch , '<' | '&' | '"') , "") , attr2 = alphabet . replace (| ch | matches ! (ch , '<' | '&' | '\'') , "") , attr_name = name , attr3 = name , pi = name , comment = alphabet , text = alphabet . replace (| ch | matches ! (ch , '<' | '&') , "") , element = name ,) ; println ! ("  - encode and write ../tests/documents/encoding/{}.xml" , enc . name ()) ; let (result , actual , _) = enc . encode (& xml) ; if enc == actual && enc != UTF_8 { write (format ! ("../tests/documents/encoding/{}.xml" , enc . name ()) , result ,) . unwrap () ; } }
};
}
