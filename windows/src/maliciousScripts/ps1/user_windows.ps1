get-process | where-object {$_.mainwindowtitle -ne ""} | Select-Object mainwindowtitle
#Gets us the windows that the user is currently working on.