Set objShell = CreateObject("WScript.Shell")

startTimer = Timer
Do
    objShell.SendKeys "{ESC}"
Loop