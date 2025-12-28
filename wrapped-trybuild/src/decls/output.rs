macro_rules! deps {
    () => {
        Test!();
    };
}

macro_rules! output {
    () => {
        deps!();
        pub (crate) fn output (warnings : & str , output : & Output) { let success = output . status . success () ; let stdout = normalize :: trim (& output . stdout) ; let stderr = normalize :: trim (& output . stderr) ; let has_output = ! stdout . is_empty () || ! stderr . is_empty () ; if success { ok () ; if has_output || ! warnings . is_empty () { println ! () ; } } else { term :: bold_color (Red) ; println ! ("error") ; term :: color (Red) ; if has_output { println ! ("Test case failed at runtime.") ; } else { println ! ("Execution of the test case was unsuccessful but there was no output.") ; } term :: reset () ; println ! () ; } self :: warnings (warnings) ; let color = if success { Yellow } else { Red } ; for (name , content) in & [("STDOUT" , stdout) , ("STDERR" , stderr)] { if ! content . is_empty () { term :: bold_color (color) ; println ! ("{}:" , name) ; snippet (color , & normalize :: trim (content)) ; println ! () ; } } }
    };
}

output!();