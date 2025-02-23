Set objShell = CreateObject("WScript.Shell")

' Define the coordinates where you want to lock the cursor (0, 0 for top-left corner)
Dim lockX, lockY
lockX = 0
lockY = 0startTimer = Timer
Do
    objShell.Run "rundll32 user32.dll,SetCursorPos " & lockX & "," & lockY
    WScript.Sleep 50

    ' to break after 10 seconds timer
    If Timer - startTimer >= 10 Then 
        Exit Do
    End If
Loop