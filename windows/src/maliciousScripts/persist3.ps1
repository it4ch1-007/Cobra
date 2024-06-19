function Copy-RegistryKey {
    param (
        [string]$sourcePath,
        [string]$destinationPath
    )

    # Ensure the destination path exists
    if (-not (Test-Path $destinationPath)) {
        New-Item -Path $destinationPath -Force | Out-Null
    }

    # Get all subkeys and values from the source path
    $sourceKey = Get-Item -Path $sourcePath

    # Copy all values from the source key to the destination key
    $sourceKey | Get-ItemProperty | ForEach-Object {
        foreach ($name in $_.PSObject.Properties.Name) {
            $value = $_.$name
            Set-ItemProperty -Path $destinationPath -Name $name -Value $value -Force
        }
    }

    # Recursively copy subkeys
    $sourceKey | Get-ChildItem | ForEach-Object {
        $newDestinationPath = Join-Path -Path $destinationPath -ChildPath $_.PSChildName
        Copy-RegistryKey -sourcePath $_.PSPath -destinationPath $newDestinationPath
    }
}

# Define the source and destination registry paths
$sourcePath = "HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon"
$destinationPath = "HKLM:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon"

# Perform the copy operation
Copy-RegistryKey -sourcePath $sourcePath -destinationPath $destinationPath
$sourcePath = "HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Notify"
$destinationPath = "HKLM\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\Notify"

# Perform the copy operation
Copy-RegistryKey -sourcePath $sourcePath -destinationPath $destinationPath
$sourcePath = "HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\shell"
$destinationPath = "HKCU\Software\Microsoft\Windows NT\CurrentVersion\Winlogon\shell"

# Perform the copy operation
Copy-RegistryKey -sourcePath $sourcePath -destinationPath $destinationPath
