macro_rules! deps {
    () => {
        DetectionState!();
    };
}

macro_rules! finalize {
    () => {
        deps!();
        # [inline] pub (super) fn finalize (state : & mut [u32 ; 5] , total : u64 , last_block : & [u8] , ctx : & mut DetectionState ,) { let mut total = total + last_block . len () as u64 ; let last = last_block . len () ; let needs_two_blocks = last >= 56 ; let mut buffer = [0u8 ; BLOCK_SIZE] ; buffer [.. last] . copy_from_slice (last_block) ; let left = BLOCK_SIZE - last ; if needs_two_blocks { let padn = 120 - last ; let (pad0 , pad1) = SHA1_PADDING [.. padn] . split_at (left) ; buffer [last ..] . copy_from_slice (pad0) ; compress (state , ctx , & [buffer]) ; buffer [.. pad1 . len ()] . copy_from_slice (pad1) ; } else { let padn = 56 - last ; buffer [last .. 56] . copy_from_slice (& SHA1_PADDING [.. padn]) ; } total <<= 3 ; buffer [56] = (total >> 56) as u8 ; buffer [57] = (total >> 48) as u8 ; buffer [58] = (total >> 40) as u8 ; buffer [59] = (total >> 32) as u8 ; buffer [60] = (total >> 24) as u8 ; buffer [61] = (total >> 16) as u8 ; buffer [62] = (total >> 8) as u8 ; buffer [63] = total as u8 ; compress (state , ctx , & [buffer]) ; }
    };
}

finalize!()