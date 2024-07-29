#This script is to exploit the capslock button of the system and blink it on invoking

function Caps-indicator {
    [CmdletBinding()]
    param(
        # This is to accept the command line arguments
        [Parameter (Mandatory = $True, ValueFromPipeline = $True)]
        [string]$pause,

        [Parameter (Mandatory = $True)]
        [int]$blinks
    )

    $o = New-Object -ComObject WScript.shell
    #This is to create a new COM object and be able to execute commands using the wscript.shell shell inside the system
    for($i = 1;$i -le $blinks * 2; $i++){
        #This is to press the keys on their own using the shell script
        $o.SendKeys("{CAPSLOCK}");Start-Sleep -Milliseconds $pause
    }
}