macro_rules! deps {
    () => {
        Data!();
        Result!();
        Palette!();
        Error!();
    };
}

macro_rules! write_diff {
    () => {
        deps!();
        pub fn write_diff (writer : & mut dyn std :: fmt :: Write , expected : & crate :: Data , actual : & crate :: Data , expected_name : Option < & dyn std :: fmt :: Display > , actual_name : Option < & dyn std :: fmt :: Display > , palette : crate :: report :: Palette ,) -> Result < () , std :: fmt :: Error > { # [allow (unused_mut)] let mut rendered = false ; # [cfg (feature = "diff")] if let (Some (expected_relevant) , Some (actual_relevant)) = (expected . relevant () , actual . relevant ()) { let expected_rendered = expected . render () . unwrap () ; let expected_line_offset = expected_rendered [.. expected_rendered . find (expected_relevant) . unwrap_or (expected_rendered . len ())] . lines () . count () ; let actual_rendered = actual . render () . unwrap () ; let actual_line_offset = actual_rendered [.. actual_rendered . find (actual_relevant) . unwrap_or (actual_rendered . len ())] . lines () . count () ; write_diff_inner (writer , expected_relevant , actual_relevant , expected_name , actual_name , palette , expected_line_offset , actual_line_offset ,) ? ; rendered = true ; } else if let (Some (expected) , Some (actual)) = (expected . render () , actual . render ()) { let expected_line_offset = 0 ; let actual_line_offset = 0 ; write_diff_inner (writer , & expected , & actual , expected_name , actual_name , palette , expected_line_offset , actual_line_offset ,) ? ; rendered = true ; } if ! rendered { if let Some (expected_name) = expected_name { writeln ! (writer , "{} {}:" , expected_name , palette . error ("(expected)")) ? ; } else { writeln ! (writer , "{}:" , palette . error ("Expected")) ? ; } writeln ! (writer , "{}" , palette . error (& expected)) ? ; if let Some (actual_name) = actual_name { writeln ! (writer , "{} {}:" , actual_name , palette . info ("(actual)")) ? ; } else { writeln ! (writer , "{}:" , palette . info ("Actual")) ? ; } writeln ! (writer , "{}" , palette . info (& actual)) ? ; } Ok (()) }
    };
}

write_diff!()