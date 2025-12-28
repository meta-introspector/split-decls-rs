macro_rules! compose_hangul {
    () => {
        # [allow (unsafe_code)] # [inline (always)] # [allow (ellipsis_inclusive_range_patterns)] fn compose_hangul (a : char , b : char) -> Option < char > { let (a , b) = (a as u32 , b as u32) ; match (a , b) { (L_BASE ..= L_LAST , V_BASE ..= V_LAST) => { let l_index = a - L_BASE ; let v_index = b - V_BASE ; let lv_index = l_index * N_COUNT + v_index * T_COUNT ; let s = S_BASE + lv_index ; Some (unsafe { char :: from_u32_unchecked (s) }) } (S_BASE ..= S_LAST , T_FIRST ..= T_LAST) if (a - S_BASE) % T_COUNT == 0 => { Some (unsafe { char :: from_u32_unchecked (a + (b - T_BASE)) }) } _ => None , } }
    };
}

compose_hangul!();