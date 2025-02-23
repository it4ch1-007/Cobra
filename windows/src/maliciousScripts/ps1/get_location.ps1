# We will be using GeoWatcher to track the location


function Get-GeoLocation{
    try{
        Add-Type -AssemblyName System.Device
        $GeoWatcher = New-Object System.Device.Location.GeoCoordinateWatcher
        # Creates the required geowatcher object
        $GeoWatcher.Start()
        # starts the location tracker

        while(($GeoWatcher.Status -ne 'Ready') -and ($GeoWatcher.Permission -ne 'Denied')) {
            Start-Sleep -Milliseconds 100
            # the geowatcher will turn off if the permissions are denied to it or else the status is not ready for the geowatcher

            if($GeoWatcher.Permission -eq 'Denied'){
                Write-Error 'Access Denied for location tracking'
            } else {
                $GL = $GeoWatcher.Position.Location | Select Latitude,Longitude
                $GL = $GL -split " "
                $Lat = $GL[0].Substring(11) -replace ".$"
                $Lon = $GL[1].Substring(10) -replace ".$"
                return $Lat, $Lon
            }
        }
    }
    catch{
        Write-Error "No coordinates found"
        return "No  coordinated found"
        -ErrorAction SilentlyContinue
    }

}