use std::arch::asm;

#[repr(C)]
struct PEB {
    reserved1: [u8; 2], 
    being_debugged: u8,
    reserved2: [u8; 57], 
}
pub unsafe fn check_debug() -> bool{
    let p_peb: *const PEB;
    
        asm!(
            "mov rax, gs:[0x60]",
            out("rax") p_peb,
            options(nomem,nostack)
        );
    
    
    (*p_peb).being_debugged!=0 //The parenthesis are reqd so that  the being_debugged is not deciphered before deref the pointer.
}