Get-Process | Where-Object { $_.ProcessName -eq "Sysmon" }#command detects if the sysmon is running on the system
Get-CimInstance win32_service -Filter "Description = 'System Monitor service'"
# or
Get-Service | where-object {$_.DisplayName -like "*sysm*"}
#some systems have different names for Sysmon process thus we have to use different commands for the same query

#We can also check the operational events of Sysmon using the command:
reg query HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\WINEVT\Channels\Microsoft-Windows-Sysmon/Operational

#We can also use this command to get all the sysinternals tools that are working on the system
ls HKCU:\Software\Sysinternals

#To read the Sysmon config file
sysmon -c

#To get the address of the config file
findstr /si '<ProcessCreate onmatch="exclude">' C:\tools\*

#Getting the sysmon process rules
(Get-SysmonConfiguration).Rules

#Basic Idea to bypass Sysmon is to see its rules and conditions and bypass them through our payload


