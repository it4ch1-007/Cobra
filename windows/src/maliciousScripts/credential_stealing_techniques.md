- `Start-Transcript -Path C:\transcript.txt` is the command to log the details of all the console commands and scripts. This gives us way too many data about the user's activity.We can use the `posh.cs` file for this that is slightly more undetectable.

{To Implement}
- Create the dump of lsass.exe that will be created in `C:\Users\Admin\APPDATA\Local\Temp\lsass.DMP` and read it using dump file tools or like `Windbg` similar tools.
    -  `procdump.exe -accepteula -ma lsass.exe lsass.dmp` is the command only for the systems having the procdump tool that dumps the lsass process.
    - `.\rundll32.exe C:\windows\System32\comsvcs.dll, MiniDump 624 C:\temp\lsass.dmp full` executed inside the `System32` directory.

{To Implement}
- MiniDumpWriteDump function API used to dump the lsass.exe process. PssCaptureSnapshot  is also similar to this API function. It gets us the snapshot of the lsass.exe process that gets us the handle and thus we pass it to the MiniDumpWriteDump API function.

{To Implement}
- `reg save hklm\sam sam` if this command works then it will dump the sam registry key.

{To Implement}
- `esentutl.exe /y /vss C:\Windows\System32\config\SAM /d c:\temp\sam` command requires privileges but will dump the SAM process details into `C:\temp\sam` directory. It can be detected by `CreateFile` and `ReadFile` operations on the SAM process.

{To Implement}(in batch command prompt)
- `reg query HKLM /f password /t REG_SZ /s` command gives us the utilities inside the whole system having the word `password` in it.

- `reg query "hklm\system\currentcontrolset\control\lsa" /v "notification packages"` command gives us the pssword filter. We have to implement this using a Password Filtering DLL.

{To be Implemented}(We will need the working of mimikaatz for this)
- In Windows 8 lower versions WDigest used to store the password in plaintext but in the newer versions we have to force the registry key to store the passwords next time the person logs in or spawn any shell.

{To be Implemented}
- Credential delegation inside the Windows is there to serve and secure the credentials of the user from the target server but when it is on we can dump the credenetials without accessing the lsass.exe process:
    - kerberos credential stealing: We need to be in the privileged command prompt and run `kekeo.exe` tool inside the victim's system.Then we have to connect to the victim's system using this tool and steal the creds.
    - NTLM creds can be stolen more easily as it can be done on the same machine where the keko instances are running with low privilege.
    - `reg query HKLM\SOFTWARE\Policies\Microsoft\Windows\CredentialsDelegation` to check if the credential delegation is turned on.
