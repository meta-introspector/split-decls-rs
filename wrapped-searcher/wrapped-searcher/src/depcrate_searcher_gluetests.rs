// Generated macro for tests (module)
macro_rules! Depcrate_searcher_gluetests {
() => {
// Module: crate::searcher::glue
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { searcher :: { BinaryDetection , SearcherBuilder } , testutil :: { KitchenSink , RegexMatcher , SearcherTester } , } ; use super :: * ; const SHERLOCK : & 'static str = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
" ; const CODE : & 'static str = "\
extern crate snap;

use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Wrap the stdin reader in a Snappy reader.
    let mut rdr = snap::Reader::new(stdin.lock());
    let mut wtr = stdout.lock();
    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");
}
" ; # [test] fn basic1 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn basic2 () { let exp = "\nbyte count:366\n" ; SearcherTester :: new (SHERLOCK , "NADA") . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn basic3 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "a") . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn basic4 () { let haystack = "\
a
b

c


d
" ; let byte_count = haystack . len () ; let exp = format ! ("0:a\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , "a") . line_number (false) . expected_no_line_number (& exp) . test () ; } # [test] fn invert1 () { let exp = "\
65:Holmeses, success in the province of detective work must always
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; } # [test] fn line_number1 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; let exp_line = "\
1:0:For the Doctor Watsons of this world, as opposed to the Sherlock
3:129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . expected_no_line_number (exp) . expected_with_line_number (exp_line) . test () ; } # [test] fn line_number_invert1 () { let exp = "\
65:Holmeses, success in the province of detective work must always
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; let exp_line = "\
2:65:Holmeses, success in the province of detective work must always
4:193:can extract a clew from a wisp of straw or a flake of cigar ash;
5:258:but Doctor Watson has to have it taken out for him and dusted,
6:321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . invert_match (true) . expected_no_line_number (exp) . expected_with_line_number (exp_line) . test () ; } # [test] fn multi_line_overlap1 () { let haystack = "xxx\nabc\ndefxxxabc\ndefxxx\nxxx" ; let byte_count = haystack . len () ; let exp = format ! ("4:abc\n8:defxxxabc\n18:defxxx\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , "abc\ndef") . by_line (false) . line_number (false) . expected_no_line_number (& exp) . test () ; } # [test] fn multi_line_overlap2 () { let haystack = "xxx\nabc\ndefabc\ndefxxx\nxxx" ; let byte_count = haystack . len () ; let exp = format ! ("4:abc\n8:defabc\n15:defxxx\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , "abc\ndef") . by_line (false) . line_number (false) . expected_no_line_number (& exp) . test () ; } # [test] fn empty_line1 () { let exp = "\nbyte count:0\n" ; SearcherTester :: new ("" , r"^$") . expected_no_line_number (exp) . expected_with_line_number (exp) . test () ; } # [test] fn empty_line2 () { let exp = "0:\n\nbyte count:1\n" ; let exp_line = "1:0:\n\nbyte count:1\n" ; SearcherTester :: new ("\n" , r"^$") . expected_no_line_number (exp) . expected_with_line_number (exp_line) . test () ; } # [test] fn empty_line3 () { let exp = "0:\n1:\n\nbyte count:2\n" ; let exp_line = "1:0:\n2:1:\n\nbyte count:2\n" ; SearcherTester :: new ("\n\n" , r"^$") . expected_no_line_number (exp) . expected_with_line_number (exp_line) . test () ; } # [test] fn empty_line4 () { let haystack = "\
a
b

c


d
" ; let byte_count = haystack . len () ; let exp = format ! ("4:\n7:\n8:\n\nbyte count:{}\n" , byte_count) ; let exp_line = format ! ("3:4:\n5:7:\n6:8:\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , r"^$") . expected_no_line_number (& exp) . expected_with_line_number (& exp_line) . test () ; } # [test] fn empty_line5 () { let haystack = "\
a
b

c


d" ; let byte_count = haystack . len () ; let exp = format ! ("4:\n7:\n8:\n\nbyte count:{}\n" , byte_count) ; let exp_line = format ! ("3:4:\n5:7:\n6:8:\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , r"^$") . expected_no_line_number (& exp) . expected_with_line_number (& exp_line) . test () ; } # [test] fn empty_line6 () { let haystack = "\
a
b

c


d

" ; let byte_count = haystack . len () ; let exp = format ! ("4:\n7:\n8:\n11:\n\nbyte count:{}\n" , byte_count) ; let exp_line = format ! ("3:4:\n5:7:\n6:8:\n8:11:\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (haystack , r"^$") . expected_no_line_number (& exp) . expected_with_line_number (& exp_line) . test () ; } # [test] fn big1 () { let mut haystack = String :: new () ; haystack . push_str ("a\n") ; for _ in 0 .. (4 * (DEFAULT_BUFFER_CAPACITY + 7)) { haystack . push_str ("zzz\n") ; } haystack . push_str ("a\n") ; let byte_count = haystack . len () ; let exp = format ! ("0:a\n1048690:a\n\nbyte count:{}\n" , byte_count) ; SearcherTester :: new (& haystack , "a") . line_number (false) . expected_no_line_number (& exp) . test () ; } # [test] fn big_error_one_line () { let mut haystack = String :: new () ; haystack . push_str ("a\n") ; for _ in 0 .. (4 * (DEFAULT_BUFFER_CAPACITY + 7)) { haystack . push_str ("zzz\n") ; } haystack . push_str ("a\n") ; let matcher = RegexMatcher :: new ("a") ; let mut sink = KitchenSink :: new () ; let mut searcher = SearcherBuilder :: new () . heap_limit (Some (3)) . build () ; let result = searcher . search_reader (& matcher , haystack . as_bytes () , & mut sink) ; assert ! (result . is_err ()) ; } # [test] fn big_error_multi_line () { let mut haystack = String :: new () ; haystack . push_str ("a\n") ; for _ in 0 .. (4 * (DEFAULT_BUFFER_CAPACITY + 7)) { haystack . push_str ("zzz\n") ; } haystack . push_str ("a\n") ; let matcher = RegexMatcher :: new ("a") ; let mut sink = KitchenSink :: new () ; let mut searcher = SearcherBuilder :: new () . multi_line (true) . heap_limit (Some (haystack . len ())) . build () ; let result = searcher . search_reader (& matcher , haystack . as_bytes () , & mut sink) ; assert ! (result . is_err ()) ; } # [test] fn binary1 () { let haystack = "\x00a" ; let exp = "\nbyte count:0\nbinary offset:0\n" ; SearcherTester :: new (haystack , "a") . binary_detection (BinaryDetection :: quit (0)) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn binary2 () { let haystack = "a\x00" ; let exp = "\nbyte count:0\nbinary offset:1\n" ; SearcherTester :: new (haystack , "a") . binary_detection (BinaryDetection :: quit (0)) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn binary3 () { let mut haystack = String :: new () ; haystack . push_str ("a\n") ; for _ in 0 .. DEFAULT_BUFFER_CAPACITY { haystack . push_str ("zzz\n") ; } haystack . push_str ("a\n") ; haystack . push_str ("zzz\n") ; haystack . push_str ("a\x00a\n") ; haystack . push_str ("zzz\n") ; haystack . push_str ("a\n") ; let exp = "0:a\n\nbyte count:262146\nbinary offset:262153\n" ; let exp_slice = "0:a\n262146:a\n\nbyte count:262153\nbinary offset:262153\n" ; SearcherTester :: new (& haystack , "a") . binary_detection (BinaryDetection :: quit (0)) . line_number (false) . auto_heap_limit (false) . expected_no_line_number (exp) . expected_slice_no_line_number (exp_slice) . test () ; } # [test] fn binary4 () { let mut haystack = String :: new () ; haystack . push_str ("a\n") ; for _ in 0 .. DEFAULT_BUFFER_CAPACITY { haystack . push_str ("zzz\n") ; } haystack . push_str ("a\n") ; haystack . push_str ("b\x00b\n") ; haystack . push_str ("a\x00a\n") ; haystack . push_str ("a\n") ; let exp = "0:a\n\nbyte count:262146\nbinary offset:262149\n" ; let exp_slice = "0:a\n262146:a\n\nbyte count:262153\nbinary offset:262153\n" ; SearcherTester :: new (& haystack , "a") . binary_detection (BinaryDetection :: quit (0)) . line_number (false) . auto_heap_limit (false) . expected_no_line_number (exp) . expected_slice_no_line_number (exp_slice) . test () ; } # [test] fn passthru_sherlock1 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . passthru (true) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn passthru_sherlock_invert1 () { let exp = "\
0-For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . passthru (true) . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock1 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;

byte count:366
" ; let exp_lines = "\
1:0:For the Doctor Watsons of this world, as opposed to the Sherlock
2-65-Holmeses, success in the province of detective work must always
3:129:be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (1) . before_context (1) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (1) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . before_context (1) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock_invert1 () { let exp = "\
0-For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
1-0-For the Doctor Watsons of this world, as opposed to the Sherlock
2:65:Holmeses, success in the province of detective work must always
3-129-be, to a very large extent, the result of luck. Sherlock Holmes
4:193:can extract a clew from a wisp of straw or a flake of cigar ash;
5:258:but Doctor Watson has to have it taken out for him and dusted,
6:321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (1) . before_context (1) . line_number (true) . invert_match (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; SearcherTester :: new (SHERLOCK , "Sherlock") . before_context (1) . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; let exp = "\
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (1) . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock2 () { let exp = "\
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
2-65-Holmeses, success in the province of detective work must always
3:129:be, to a very large extent, the result of luck. Sherlock Holmes
4:193:can extract a clew from a wisp of straw or a flake of cigar ash;
5-258-but Doctor Watson has to have it taken out for him and dusted,
6:321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , " a ") . after_context (1) . before_context (1) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; SearcherTester :: new (SHERLOCK , " a ") . before_context (1) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
129:be, to a very large extent, the result of luck. Sherlock Holmes
193:can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , " a ") . after_context (1) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock_invert2 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
1:0:For the Doctor Watsons of this world, as opposed to the Sherlock
2:65:Holmeses, success in the province of detective work must always
3-129-be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;
5:258:but Doctor Watson has to have it taken out for him and dusted,
6-321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , " a ") . after_context (1) . before_context (1) . line_number (true) . invert_match (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
--
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,

byte count:366
" ; SearcherTester :: new (SHERLOCK , " a ") . before_context (1) . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
--
258:but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , " a ") . after_context (1) . line_number (false) . invert_match (true) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock3 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,

byte count:366
" ; let exp_lines = "\
1:0:For the Doctor Watsons of this world, as opposed to the Sherlock
2-65-Holmeses, success in the province of detective work must always
3:129:be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;
5-258-but Doctor Watson has to have it taken out for him and dusted,

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock4 () { let exp = "\
129-be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
3-129-be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;
5:258:but Doctor Watson has to have it taken out for him and dusted,
6-321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "dusted") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
258:but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "dusted") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
129-be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258:but Doctor Watson has to have it taken out for him and dusted,

byte count:366
" ; SearcherTester :: new (SHERLOCK , "dusted") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock5 () { let exp = "\
0-For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
1-0-For the Doctor Watsons of this world, as opposed to the Sherlock
2:65:Holmeses, success in the province of detective work must always
3-129-be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;
5-258-but Doctor Watson has to have it taken out for him and dusted,
6:321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "success|attached") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
65:Holmeses, success in the province of detective work must always
129-be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
--
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "success|attached") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
0-For the Doctor Watsons of this world, as opposed to the Sherlock
65:Holmeses, success in the province of detective work must always
--
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321:and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "success|attached") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_sherlock6 () { let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; let exp_lines = "\
1:0:For the Doctor Watsons of this world, as opposed to the Sherlock
2-65-Holmeses, success in the province of detective work must always
3:129:be, to a very large extent, the result of luck. Sherlock Holmes
4-193-can extract a clew from a wisp of straw or a flake of cigar ash;
5-258-but Doctor Watson has to have it taken out for him and dusted,
6-321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (3) . before_context (3) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes
193-can extract a clew from a wisp of straw or a flake of cigar ash;
258-but Doctor Watson has to have it taken out for him and dusted,
321-and exhibited clearly, with a label attached.
byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . after_context (3) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
0:For the Doctor Watsons of this world, as opposed to the Sherlock
65-Holmeses, success in the province of detective work must always
129:be, to a very large extent, the result of luck. Sherlock Holmes

byte count:366
" ; SearcherTester :: new (SHERLOCK , "Sherlock") . before_context (3) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_code1 () { let exp = "\
33-
34-fn main() {
46:    let stdin = io::stdin();
75-    let stdout = io::stdout();
106-
107:    // Wrap the stdin reader in a Snappy reader.
156:    let mut rdr = snap::Reader::new(stdin.lock());
207-    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; let exp_lines = "\
4-33-
5-34-fn main() {
6:46:    let stdin = io::stdin();
7-75-    let stdout = io::stdout();
8-106-
9:107:    // Wrap the stdin reader in a Snappy reader.
10:156:    let mut rdr = snap::Reader::new(stdin.lock());
11-207-    let mut wtr = stdout.lock();
12-240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; SearcherTester :: new (CODE , "stdin") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
46:    let stdin = io::stdin();
75-    let stdout = io::stdout();
106-
107:    // Wrap the stdin reader in a Snappy reader.
156:    let mut rdr = snap::Reader::new(stdin.lock());
207-    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; SearcherTester :: new (CODE , "stdin") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
33-
34-fn main() {
46:    let stdin = io::stdin();
75-    let stdout = io::stdout();
106-
107:    // Wrap the stdin reader in a Snappy reader.
156:    let mut rdr = snap::Reader::new(stdin.lock());

byte count:307
" ; SearcherTester :: new (CODE , "stdin") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_code2 () { let exp = "\
34-fn main() {
46-    let stdin = io::stdin();
75:    let stdout = io::stdout();
106-
107-    // Wrap the stdin reader in a Snappy reader.
156-    let mut rdr = snap::Reader::new(stdin.lock());
207:    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");
305-}

byte count:307
" ; let exp_lines = "\
5-34-fn main() {
6-46-    let stdin = io::stdin();
7:75:    let stdout = io::stdout();
8-106-
9-107-    // Wrap the stdin reader in a Snappy reader.
10-156-    let mut rdr = snap::Reader::new(stdin.lock());
11:207:    let mut wtr = stdout.lock();
12-240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");
13-305-}

byte count:307
" ; SearcherTester :: new (CODE , "stdout") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
75:    let stdout = io::stdout();
106-
107-    // Wrap the stdin reader in a Snappy reader.
--
207:    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");
305-}

byte count:307
" ; SearcherTester :: new (CODE , "stdout") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
34-fn main() {
46-    let stdin = io::stdin();
75:    let stdout = io::stdout();
--
107-    // Wrap the stdin reader in a Snappy reader.
156-    let mut rdr = snap::Reader::new(stdin.lock());
207:    let mut wtr = stdout.lock();

byte count:307
" ; SearcherTester :: new (CODE , "stdout") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn context_code3 () { let exp = "\
20-use std::io;
33-
34:fn main() {
46-    let stdin = io::stdin();
75-    let stdout = io::stdout();
106-
107-    // Wrap the stdin reader in a Snappy reader.
156:    let mut rdr = snap::Reader::new(stdin.lock());
207-    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; let exp_lines = "\
3-20-use std::io;
4-33-
5:34:fn main() {
6-46-    let stdin = io::stdin();
7-75-    let stdout = io::stdout();
8-106-
9-107-    // Wrap the stdin reader in a Snappy reader.
10:156:    let mut rdr = snap::Reader::new(stdin.lock());
11-207-    let mut wtr = stdout.lock();
12-240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; SearcherTester :: new (CODE , "fn main|let mut rdr") . after_context (2) . before_context (2) . line_number (true) . expected_no_line_number (exp) . expected_with_line_number (exp_lines) . test () ; let exp = "\
34:fn main() {
46-    let stdin = io::stdin();
75-    let stdout = io::stdout();
--
156:    let mut rdr = snap::Reader::new(stdin.lock());
207-    let mut wtr = stdout.lock();
240-    io::copy(&mut rdr, &mut wtr).expect(\"I/O operation failed\");

byte count:307
" ; SearcherTester :: new (CODE , "fn main|let mut rdr") . after_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; let exp = "\
20-use std::io;
33-
34:fn main() {
--
106-
107-    // Wrap the stdin reader in a Snappy reader.
156:    let mut rdr = snap::Reader::new(stdin.lock());

byte count:307
" ; SearcherTester :: new (CODE , "fn main|let mut rdr") . before_context (2) . line_number (false) . expected_no_line_number (exp) . test () ; } # [test] fn scratch () { use crate :: sinks ; use crate :: testutil :: RegexMatcher ; const SHERLOCK : & 'static [u8] = b"\
For the Doctor Wat\xFFsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.\
    " ; let haystack = SHERLOCK ; let matcher = RegexMatcher :: new ("Sherlock") ; let mut searcher = SearcherBuilder :: new () . line_number (true) . build () ; searcher . search_reader (& matcher , haystack , sinks :: Lossy (| n , line | { print ! ("{}:{}" , n , line) ; Ok (true) }) ,) . unwrap () ; } # [test] fn regression_2260 () { use grep_regex :: RegexMatcherBuilder ; use crate :: SearcherBuilder ; let matcher = RegexMatcherBuilder :: new () . line_terminator (Some (b'\n')) . build (r"^\w+$") . unwrap () ; let mut searcher = SearcherBuilder :: new () . line_number (true) . build () ; let mut matched = false ; searcher . search_slice (& matcher , b"GATC\n" , crate :: sinks :: UTF8 (| _ , _ | { matched = true ; Ok (true) }) ,) . unwrap () ; assert ! (matched) ; } }
};
}
