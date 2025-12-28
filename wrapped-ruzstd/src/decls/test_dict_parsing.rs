macro_rules! deps {
    () => {
        Dictionary!();
    };
}

macro_rules! test_dict_parsing {
    () => {
        deps!();
        # [test] fn test_dict_parsing () { use crate :: decoding :: dictionary :: Dictionary ; use alloc :: vec ; let mut raw = vec ! [0u8 ; 8] ; raw [0] = 0x37 ; raw [1] = 0xA4 ; raw [2] = 0x30 ; raw [3] = 0xEC ; let dict_id = 0x47232101 ; raw [4] = 0x01 ; raw [5] = 0x21 ; raw [6] = 0x23 ; raw [7] = 0x47 ; let raw_tables = & [54 , 16 , 192 , 155 , 4 , 0 , 207 , 59 , 239 , 121 , 158 , 116 , 220 , 93 , 114 , 229 , 110 , 41 , 249 , 95 , 165 , 255 , 83 , 202 , 254 , 68 , 74 , 159 , 63 , 161 , 100 , 151 , 137 , 21 , 184 , 183 , 189 , 100 , 235 , 209 , 251 , 174 , 91 , 75 , 91 , 185 , 19 , 39 , 75 , 146 , 98 , 177 , 249 , 14 , 4 , 35 , 0 , 0 , 0 , 40 , 40 , 20 , 10 , 12 , 204 , 37 , 196 , 1 , 173 , 122 , 0 , 4 , 0 , 128 , 1 , 2 , 2 , 25 , 32 , 27 , 27 , 22 , 24 , 26 , 18 , 12 , 12 , 15 , 16 , 11 , 69 , 37 , 225 , 48 , 20 , 12 , 6 , 2 , 161 , 80 , 40 , 20 , 44 , 137 , 145 , 204 , 46 , 0 , 0 , 0 , 0 , 0 , 116 , 253 , 16 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,] ; raw . extend (& raw_tables [..]) ; raw . extend (vec ! [3 , 0 , 0 , 0]) ; raw . extend (vec ! [10 , 0 , 0 , 0]) ; raw . extend (vec ! [0xEF , 0xCD , 0xAB , 0]) ; let raw_content = vec ! [1 , 1 , 1 , 1 , 1 , 2 , 2 , 2 , 2 , 2 , 2 , 1 , 1 , 123 , 3 , 234 , 23 , 234 , 34 , 23 , 234 , 34 , 34 , 234 , 234 ,] ; raw . extend (& raw_content) ; let dict = Dictionary :: decode_dict (& raw) . unwrap () ; if dict . id != dict_id { panic ! ("Dict-id did not get parsed correctly. Is: {}, Should be: {}" , dict . id , dict_id) ; } if ! dict . dict_content . eq (& raw_content) { panic ! ("dict content did not get parsed correctly. Is: {:?}, Should be: {:?}" , dict . dict_content , raw_content) ; } if ! dict . offset_hist . eq (& [3 , 10 , 0x00ABCDEF]) { panic ! ("offset history did not get parsed correctly. Is: {:?}, Should be: {:?}" , dict . offset_hist , [3 , 10 , 0x00ABCDEF]) ; } raw [0] = 1 ; raw [1] = 1 ; raw [2] = 1 ; raw [3] = 1 ; match Dictionary :: decode_dict (& raw) { Ok (_) => panic ! ("The dict got decoded but the magic num was incorrect!") , Err (_) => { } } }
    };
}

test_dict_parsing!()