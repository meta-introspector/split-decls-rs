macro_rules! deps {
    () => {
        In!();
    };
}

macro_rules! escape_html {
    () => {
        deps!();
        # [doc = " Escape HTML following [OWASP](https://www.owasp.org/index.php/XSS_(Cross_Site_Scripting)_Prevention_Cheat_Sheet)"] # [doc = ""] # [doc = " Escape the following characters with HTML entity encoding to prevent switching"] # [doc = " into any execution context, such as script, style, or event handlers. Using"] # [doc = " hex entities is recommended in the spec. In addition to the 5 characters"] # [doc = " significant in XML (&, <, >, \", '), the forward slash is included as it helps"] # [doc = " to end an HTML entity."] # [doc = ""] # [doc = " ```text"] # [doc = " & --> &amp;"] # [doc = " < --> &lt;"] # [doc = " > --> &gt;"] # [doc = " \" --> &quot;"] # [doc = " ' --> &#x27;     &apos; is not recommended"] # [doc = " / --> &#x2F;     forward slash is included as it helps end an HTML entity"] # [doc = " ```"] # [inline] pub fn escape_html (input : & str) -> String { let mut output = String :: with_capacity (input . len () * 2) ; for c in input . chars () { match c { '&' => output . push_str ("&amp;") , '<' => output . push_str ("&lt;") , '>' => output . push_str ("&gt;") , '"' => output . push_str ("&quot;") , '\'' => output . push_str ("&#x27;") , '/' => output . push_str ("&#x2F;") , _ => output . push (c) , } } output }
    };
}

escape_html!();