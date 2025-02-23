# this script is to retrieve the data from the browsers
function Get-BrowserData{
    #We are giving the browser name and the datatype we want as the paramters of the function
    param(
        [Parameter (Position=1,Mandatory = $True)]
        [string]$Browser,
        [Parameter (Position = 1,Mandatory = $True)]
        [string]$DataType
        #What type of data we want to extract from which browser
    )
        $Regex = '(http|https)://([\w-]+\.)+[\w-]+(/[\w- ./?%&=]*)*?'


        # This is to define the local paths in the system where the different types of information are stored 
        if     ($Browser -eq 'chrome'  -and $DataType -eq 'history'   )  {$Path = "$Env:USERPROFILE\AppData\Local\Google\Chrome\User Data\Default\History"}
        elseif ($Browser -eq 'chrome'  -and $DataType -eq 'bookmarks' )  {$Path = "$Env:USERPROFILE\AppData\Local\Google\Chrome\User Data\Default\Bookmarks"}
        elseif ($Browser -eq 'edge'    -and $DataType -eq 'history'   )  {$Path = "$Env:USERPROFILE\AppData\Local\Microsoft/Edge/User Data/Default/History"}
        elseif ($Browser -eq 'edge'    -and $DataType -eq 'bookmarks' )  {$Path = "$env:USERPROFILE/AppData/Local/Microsoft/Edge/User Data/Default/Bookmarks"}
        elseif ($Browser -eq 'firefox' -and $DataType -eq 'history'   )  {$Path = "$Env:USERPROFILE\AppData\Roaming\Mozilla\Firefox\Profiles\*.default-release\places.sqlite"}


        
        $Value= Get-Content -Path $Path | Select-String -AllMatches $regex |% {($_.Matches).Value} |Sort -Unique
        $Value | ForEach-Object {
                $Key = $_
                if ($Key -match $Search){
                    New-Object -TypeName PSObject -Property @{
                        User= $env:UserName
                        Browser = $Browser
                        DataType = $DataType
                        Data = $_  
                    }
                }
            }
}