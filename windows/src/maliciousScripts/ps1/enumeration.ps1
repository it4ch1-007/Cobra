function hunt() {
    [CmdletBinding()]Param()
    $commandlines = Import-Csv C:\Users\akshi\Downloads\cmd-test.csv
    $watch = 'whoami|net1 user|hostname|netstat|net localgroup|cmd /c'
    $matchedCommandlines = $commandlines| where-object {  $_."event_data.CommandLine" -match $watch}

    $matchedCommandlines| foreach-Object {
        [datetime]$eventTime = $_."@timestamp"
        [datetime]$low = $eventTime.AddSeconds(-60)
        [datetime]$high = $eventTime.AddSeconds(60)
        $clusteredCommandlines = $commandlines | Where-Object { [datetime]$_."@timestamp" -ge $low -and [datetime]$_."@timestamp" -le $high -and  $_."event_data.CommandLine" -match $watch}
        
        if ($clusteredCommandlines.length -ge 4) {
            Write-Verbose "Possible enumeration around time: $low - $high ($eventTime)"
            $clusteredCommandlines
        }
    }
}