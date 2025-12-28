macro_rules! deps {
    () => {
        DataInner!();
        Data!();
        Palette!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use super :: * ; # [cfg (feature = "diff")] # [test] fn diff_eq () { let expected = "Hello\nWorld\n" ; let expected_name = "A" ; let actual = "Hello\nWorld\n" ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff_inner (& mut actual_diff , expected , actual , Some (& expected_name) , Some (& actual_name) , palette , 0 , 0 ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   1    1 | Hello
   2    2 | World
" ; assert_eq ! (expected_diff , actual_diff) ; } # [cfg (feature = "diff")] # [test] fn diff_ne_line_missing () { let expected = "Hello\nWorld\n" ; let expected_name = "A" ; let actual = "Hello\n" ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff_inner (& mut actual_diff , expected , actual , Some (& expected_name) , Some (& actual_name) , palette , 0 , 0 ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   1    1 | Hello
   2      - World
" ; assert_eq ! (expected_diff , actual_diff) ; } # [cfg (feature = "diff")] # [test] fn diff_eq_trailing_extra_newline () { let expected = "Hello\nWorld" ; let expected_name = "A" ; let actual = "Hello\nWorld\n" ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff_inner (& mut actual_diff , expected , actual , Some (& expected_name) , Some (& actual_name) , palette , 0 , 0 ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   1    1 | Hello
   2      - World∅
        2 + World
" ; assert_eq ! (expected_diff , actual_diff) ; } # [cfg (feature = "diff")] # [test] fn diff_eq_trailing_newline_missing () { let expected = "Hello\nWorld\n" ; let expected_name = "A" ; let actual = "Hello\nWorld" ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff_inner (& mut actual_diff , expected , actual , Some (& expected_name) , Some (& actual_name) , palette , 0 , 0 ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   1    1 | Hello
   2      - World
        2 + World∅
" ; assert_eq ! (expected_diff , actual_diff) ; } # [cfg (feature = "diff")] # [test] fn diff_eq_elided () { let mut expected = String :: new () ; expected . push_str ("Hello\n") ; for i in 0 .. 20 { expected . push_str (& i . to_string ()) ; expected . push ('\n') ; } expected . push_str ("World\n") ; for i in 0 .. 20 { expected . push_str (& i . to_string ()) ; expected . push ('\n') ; } expected . push_str ("!\n") ; let expected_name = "A" ; let mut actual = String :: new () ; actual . push_str ("Goodbye\n") ; for i in 0 .. 20 { actual . push_str (& i . to_string ()) ; actual . push ('\n') ; } actual . push_str ("Moon\n") ; for i in 0 .. 20 { actual . push_str (& i . to_string ()) ; actual . push ('\n') ; } actual . push_str ("?\n") ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff_inner (& mut actual_diff , & expected , & actual , Some (& expected_name) , Some (& actual_name) , palette , 0 , 0 ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   1      - Hello
        1 + Goodbye
   2    2 | 0
   3    3 | 1
   4    4 | 2
   5    5 | 3
   6    6 | 4
          ⋮
  17   17 | 15
  18   18 | 16
  19   19 | 17
  20   20 | 18
  21   21 | 19
  22      - World
       22 + Moon
  23   23 | 0
  24   24 | 1
  25   25 | 2
  26   26 | 3
  27   27 | 4
          ⋮
  38   38 | 15
  39   39 | 16
  40   40 | 17
  41   41 | 18
  42   42 | 19
  43      - !
       43 + ?
" ; assert_eq ! (expected_diff , actual_diff) ; } # [cfg (feature = "diff")] # [cfg (feature = "term-svg")] # [test] fn diff_ne_ignore_irrelevant_details () { let expected = "<svg width='100px' height='200px'>
<text>
Hello Moon
</text>
</svg>" ; let expected_name = "A" ; let actual = "<svg width='200px' height='400px'>
<text>
Hello World
</text>
</svg>" ; let actual_name = "B" ; let palette = crate :: report :: Palette :: plain () ; let mut actual_diff = String :: new () ; write_diff (& mut actual_diff , & crate :: Data :: with_inner (crate :: data :: DataInner :: TermSvg (expected . to_owned ())) , & crate :: Data :: with_inner (crate :: data :: DataInner :: TermSvg (actual . to_owned ())) , Some (& expected_name) , Some (& actual_name) , palette ,) . unwrap () ; let expected_diff = "
---- expected: A
++++ actual:   B
   2    2 | <text>
   3      - Hello Moon
        3 + Hello World
   4    4 | </text>
" ; assert_eq ! (expected_diff , actual_diff) ; } }
    };
}

test!()