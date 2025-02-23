cmd /c wmic service get name,displayname,pathname,startmode |findstr /i "auto" |findstr /i /v "c:\windows\\" |findstr /i /v ""
#This command searched for all misconfigured command line processes inside the system 
#These processes can be easily exploited.
