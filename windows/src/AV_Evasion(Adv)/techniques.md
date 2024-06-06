## Techniques

- We can use the Metasploit framework binaries and custom binaries to get the less detected shellcodes.

- We can use the cobalt strike payload if we are using any shellcode.

- We can also prefer to send the shellcode via the attacker's machine rather than storing it on the victim's machine virus.

- It is beneficial to impersonate the `lsass.exe`  process which is originally present inside the `Windows\System32` folder. It is the process that handles the access tokens and the security policies inside the system.

- API Hashing (To be Implemented){New algorithm maybe AES}

- Detecting the hook syscalls
    - The hook syscalls are placed inside the code by inserting `jmp` instruction inside the code base.
    - The AV places a `jmp` call to jump to its own dll syscall that checks the malware and thus if the RVA of the original dll in that area like `kernel32dll.dll` with respect to the EAT then we can get the original address of the dll using the base physical address of that dll. If the present address of the dll does'nt match to the original address then it is hooked by the AV for checking.


- We can also use the syscalls for the Nt native functions inside the code using assembly instead of direct functions.

{To be implemented}
- PPID Spoofing is the techinique to make a fake parent process set for the malicious process.It can be detected using logman.

- We can also prevent the third party apps to inject dlls into our malware.

{To be implemented}
- We can enable ProcessDynamicCodePolicy that can make our malware untouchable by the AVs that may try to modify or execute arbitrary instructions on our malware. The catch is that it doesnot disable the remote processes from arbitrary code execution and injection.


- PEB overwrite ;
    - We are going to hide our process as a legitimate process by overwriting its PEB that is writable from the user-space.
    - PEB->ProcessParameters(0x020)->_RTL_USER_PROCESS_PARAMTERS has the info about the process that can be changed for this camouflage.
    - `ImagePathName` changes the basic name of the process in all the tools of the windows like SysInternals.
    - 

