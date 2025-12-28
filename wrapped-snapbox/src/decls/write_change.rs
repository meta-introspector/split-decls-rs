macro_rules! deps {
    () => {
        Error!();
        Styled!();
        Palette!();
        Result!();
    };
}

macro_rules! write_change {
    () => {
        deps!();
        # [cfg (feature = "diff")] # [allow (clippy :: too_many_arguments)] fn write_change (writer : & mut dyn std :: fmt :: Write , change : similar :: InlineChange < '_ , str > , sign : & str , em_style : crate :: report :: Style , style : crate :: report :: Style , palette : crate :: report :: Palette , expected_line_offset : usize , actual_line_offset : usize ,) -> Result < () , std :: fmt :: Error > { if let Some (index) = change . old_index () { write ! (writer , "{:>4} " , palette . hint (index + 1 + expected_line_offset) ,) ? ; } else { write ! (writer , "{:>4} " , " " ,) ? ; } if let Some (index) = change . new_index () { write ! (writer , "{:>4} " , palette . hint (index + 1 + actual_line_offset) ,) ? ; } else { write ! (writer , "{:>4} " , " " ,) ? ; } write ! (writer , "{} " , Styled :: new (sign , style)) ? ; for & (emphasized , change) in change . values () { let cur_style = if emphasized { em_style } else { style } ; write ! (writer , "{}" , Styled :: new (change , cur_style)) ? ; } if change . missing_newline () { writeln ! (writer , "{}" , Styled :: new ("∅" , em_style)) ? ; } Ok (()) }
    };
}

write_change!();