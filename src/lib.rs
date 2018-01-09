/*
To generate the checksum for an ASCII string to be transmitted, clear the checksum value (CC = 0;)
Add each byte of the string to be transmitted to the checksum value (CC += ASCII byte).
Do a two’s compliment of the checksum (CC = (CC ^ 0xff) + 1;).
Convert the checksum’s upper and lower nibble’s to ASCII hex.
Send a carriage return (0x0D) and line feed (0x0A).
The following is an example C code for sending the checksum after building the initial checksum value.
*/
use std::env;
use std::process;
use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io;
use std::str;
use std::error::Error;


pub struct TelnetClient {
    address: String,
    port: u16
}
impl TelnetClient {
    pub fn new(address: String, port: u16) -> TelnetClient {
        TelnetClient {
            address: address,
            port: port,
        }
    }
    pub fn send_msg(&self, message: String) -> Result<String, Box<Error>> {
        println!("Address: {}", &self.address);

        let address = format!("{}:{}", &self.address, &self.port);
        let mut stream = TcpStream::connect(address)?;
        stream.write(message.as_bytes());

        let mut client_buffer = [0u8; 11];
        stream.read(&mut client_buffer);
        io::stdout().write(&client_buffer)?;
		io::stdout().flush()?;

        let message: &str = str::from_utf8(&client_buffer)?;
        //let another: String = message.into();
        let another: String = String::from(message);
        Ok(another)
    }
}
pub fn generate_checksum(codes: &str) -> String {
    let mut sumtotal: i32 = codes.as_bytes().iter().map(|x| x.clone() as i32).sum();
    sumtotal = (sumtotal ^ 0xff) + 1;
    let upper = (sumtotal >> 4) & 0x0f;
    let lower = sumtotal & 0x0f;
    format!("{:x}{:x}", upper, lower).to_uppercase()
}

#[test]
fn test_checksum() {
    let code = "0AZC002200".to_string();
    let checksum = generate_checksum(&code);
    assert_eq!(checksum, "CE".to_string());
}
