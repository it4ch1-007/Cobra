$o = [activator]::CreateInstance([type]::GetTypeFromCLSID("093FF999-1EA0-4079-9525-9614C3504B74"))
#This creates an instance of the COM process on the system and thus it can give us all the info we need.

$o.EnumNetworkDrives()

$o
#It can be used as a structure like PEB.