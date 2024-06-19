# Define the registry path and new command value
$registryPath = "HKCR:\txtfilelegacy\shell\printto\command"
$newCommand = "notepad.exe `"%1`""

# Set the new command for the registry key
Set-ItemProperty -Path $registryPath -Name "(Default)" -Value $newCommand -Force

# Verify the updated command (optional)
Get-ItemProperty -Path $registryPath | Select-Object -ExpandProperty "(Default)"