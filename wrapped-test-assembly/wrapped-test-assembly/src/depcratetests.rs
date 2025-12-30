// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_demangle () { use super :: * ; let before = r#"
   movw    r10, :lower16:(L__ZN16objc2_foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17hb82d9a01a97e5b26E$non_lazy_ptr-(LPC5_0+8))
   movt    r10, :upper16:(L__ZN16objc2_foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17hb82d9a01a97e5b26E$non_lazy_ptr-(LPC5_0+8))

   .section __DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
   .p2align    2, 0x0
L__ZN16objc2_foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17hb82d9a01a97e5b26E$non_lazy_ptr:
   .indirect_symbol    __ZN16objc2_foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17hb82d9a01a97e5b26E
   .long   0
        "# ; let after = r#"
   movw    r10, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
   movt    r10, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))

   .section __DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
   .p2align    2, 0x0
LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
   .indirect_symbol    SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
   .long   0
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
    .section    __TEXT,__text,regular,pure_instructions
    .intel_syntax noprefix
    .globl  _handle_with_sel
    .p2align    4, 0x90
_handle_with_sel:
    push    rbp
    mov rbp, rsp
    lea rsi, [rip + __RNvNvCslgFcLFxF7mp_24test_msg_send_static_sel15handle_with_sel9NAME_DATA]
    pop rbp
    jmp _objc_msgSend

    .section    __TEXT,__objc_methname,cstring_literals
__RNvNvCslgFcLFxF7mp_24test_msg_send_static_sel15handle_with_sel9NAME_DATA:
    .asciz  "someSelector"
        "# ; let after = r#"
    .section    __TEXT,__text,regular,pure_instructions
    .intel_syntax noprefix
    .globl  _handle_with_sel
    .p2align    4, 0x90
_handle_with_sel:
    push    rbp
    mov rbp, rsp
    lea rsi, [rip + SYM(test_msg_send_static_sel[CRATE_ID]::handle_with_sel::NAME_DATA, 0)]
    pop rbp
    jmp _objc_msgSend

    .section    __TEXT,__objc_methname,cstring_literals
SYM(test_msg_send_static_sel[CRATE_ID]::handle_with_sel::NAME_DATA, 0):
    .asciz  "someSelector"
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
_get_ascii:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
Lloh0:
    adrp    x0, l___unnamed_1@PAGE
Lloh1:
    add x0, x0, l___unnamed_1@PAGEOFF
    mov w1, #3
    bl  __ZN16objc2_foundation14__string_macro8is_ascii17h6ed9b17e599aba93E
    tbz w0, #0, LBB0_2
Lloh2:
    adrp    x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_ascii8CFSTRING@PAGE
Lloh3:
    add x0, x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_ascii8CFSTRING@PAGEOFF
    ldp x29, x30, [sp], #16
    b   __ZN16objc2_foundation14__string_macro13CFStringAscii6as_ptr17hb04bc801907abfefE
LBB0_2:
Lloh4:
    adrp    x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_asciis_8CFSTRING@PAGE
Lloh5:
    add x0, x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_asciis_8CFSTRING@PAGEOFF
    ldp x29, x30, [sp], #16
    b   __ZN16objc2_foundation14__string_macro13CFStringUtf166as_ptr17h2d998f5fc92d4caaE
    .loh AdrpAdd    Lloh0, Lloh1
    .loh AdrpAdd    Lloh2, Lloh3
    .loh AdrpAdd    Lloh4, Lloh5
        "# ; let after = r#"
_get_ascii:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
Lloh0:
    adrp    x0, l___unnamed_1@PAGE
Lloh1:
    add x0, x0, l___unnamed_1@PAGEOFF
    mov w1, #3
    bl  SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
    tbz w0, #0, LBB0_2
Lloh2:
    adrp    x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGE
Lloh3:
    add x0, x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGEOFF
    ldp x29, x30, [sp], #16
    b   SYM(objc2_foundation::__string_macro::CFStringAscii::as_ptr::GENERATED_ID, 0)
LBB0_2:
Lloh4:
    adrp    x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGE
Lloh5:
    add x0, x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGEOFF
    ldp x29, x30, [sp], #16
    b   SYM(objc2_foundation::__string_macro::CFStringUtf16::as_ptr::GENERATED_ID, 0)
    .loh AdrpAdd    Lloh0, Lloh1
    .loh AdrpAdd    Lloh2, Lloh3
    .loh AdrpAdd    Lloh4, Lloh5
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
    bl  __ZN120_$LT$objc2..__macro_helpers..RetainSemantics$LT$_$C$_$C$_$C$_$GT$$u20$as$u20$objc2..__macro_helpers..MsgSendIdFailed$GT$6failed17h6e2744dc261913f0E
        "# ; let after = r#"
    bl  SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
    .section    .bss._RNvNvCseMIdOpHE7C6_14test_ns_string9get_utf1615CACHED_NSSTRING.0,"aw",@nobits
        "# ; let after = r#"
    .section    .bss.SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,"aw",@nobits
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
    lea rdx, [rip + l_anon.a9da382cd71626477b56696a19e9dcbe.1]
    .section    __TEXT,__const
l_anon.7ff1dd02f36078179aa2659299baa0de.0:
        "# ; let after = r#"
    lea rdx, [rip + l_anon.[ID].1]
    .section    __TEXT,__const
l_anon.[ID].0:
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"
.LBB1_1:
    lea rdx, [rip + .Lanon.3eb1462e831469f1d556358e5ee820de.1]
    mov rdi, rbx
        "# ; let after = r#"
.LBB1_1:
    lea rdx, [rip + .Lanon.[ID].1]
    mov rdi, rbx
        "# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; let before = r#"__RINvNtCshB9ITk7tvJd_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtCsaWDm3USgSkM_5objc22rc2id2IdNtNtB19_7runtime6ObjectNtNtB17_9ownership6SharedEEECs8tAMaYSsbuV_19test_out_parameters:"# ; let after = r#"SYM(core[CRATE_ID]::ptr::drop_in_place::<core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::id::Id<objc2[CRATE_ID]::runtime::Object, objc2[CRATE_ID]::rc::ownership::Shared>>>, 0):"# ; let output = demangle_assembly (before) ; assert_eq ! (output , after , "Got {output}") ; } }
};
}
