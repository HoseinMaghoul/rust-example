extern crate mac_address;

use local_ip_address::local_ip;
use mac_address::{get_mac_address, MacAddress};
use std::io;







fn main() {
    println!("welcome to application");

   
    system_address::get_host_name();
    system_address::get_ip_address();
    system_address::for_mac_addres();
    
}


pub mod system_address {
    extern crate mac_address;
    use std::io;
    use std::net::IpAddr;
    use local_ip_address::local_ip;
    use local_ip_address::Error;
    use std::io::Error as io_error;



    pub fn get_host_name() -> io::Result<String> {
        dbg!(hostname::get()?);

        Ok("ok".to_string())
    }


    pub fn  get_ip_address() {
        let my_local_ip: Result<IpAddr, Error> = local_ip();


        if let Ok(my_local_ip) = my_local_ip {
            println!("This is my local IP address: {:?}", my_local_ip);
        } else {
            println!("Error getting local IP: {:?}", my_local_ip);
        }

        
    }



    pub fn for_mac_addres() {
        match mac_address::get_mac_address() {
            Ok(Some(ma)) => {
                println!("Mac address: {}", ma)
            }
            Ok(None) => println!("No mac address"),
            Err(e) => println!("{:?}", e)
        }
    }


}

 




