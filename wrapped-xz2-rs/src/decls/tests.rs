macro_rules! deps {
    () => {
        XzDecoder!();
        XzEncoder!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: { XzDecoder , XzEncoder } ; use std :: io :: prelude :: * ; use std :: iter :: repeat ; # [test] fn smoke () { let d = XzDecoder :: new (Vec :: new ()) ; let mut c = XzEncoder :: new (d , 6) ; c . write_all (b"12834") . unwrap () ; let s = repeat ("12345") . take (100000) . collect :: < String > () ; c . write_all (s . as_bytes ()) . unwrap () ; let data = c . finish () . unwrap () . finish () . unwrap () ; assert_eq ! (& data [0 .. 5] , b"12834") ; assert_eq ! (data . len () , 500005) ; assert ! (format ! ("12834{}" , s) . as_bytes () == &* data) ; } # [test] fn write_empty () { let d = XzDecoder :: new (Vec :: new ()) ; let mut c = XzEncoder :: new (d , 6) ; c . write (b"") . unwrap () ; let data = c . finish () . unwrap () . finish () . unwrap () ; assert_eq ! (& data [..] , b"") ; } # [test] fn qc () { :: quickcheck :: quickcheck (test as fn (_) -> _) ; fn test (v : Vec < u8 >) -> bool { let w = XzDecoder :: new (Vec :: new ()) ; let mut w = XzEncoder :: new (w , 6) ; w . write_all (& v) . unwrap () ; v == w . finish () . unwrap () . finish () . unwrap () } } }
    };
}

tests!()