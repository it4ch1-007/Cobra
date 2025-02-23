function speak{
    [CmdletBinding()]
    param(
        [Parameter (Position=0,Mandatory = $true,ValueFromPipeline = $true)]
        [string]$Sentence
    )

    $s.Voice = $s.GetVoices().Item(0)
    $s = New-Object -ComObject SAPI.SpVoice
    #The script uses the speech API to speak through the speakers
    $s.Rate = -2
    $s.Speak($Sentence)
}