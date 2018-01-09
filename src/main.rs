extern crate nessm1;

use nessm1::generate_checksum;
use nessm1::TelnetClient;
extern crate tiny_http;
use std::env;

enum StatusUpdate {
    ZoneChange(ZoneChangeUpdate)
}
struct ZoneChangeUpdate {
    //Example: 0AZCZZZS00CC
    length: String,         //0A
    zc_command: String,     //ZC
    zone_number: String,    //ZZZ
    zone_status: String,    //S
    future_use: String,     //00
    checksum: String        //CC
}
fn main() {
    //09kf02100D3 <-- keypad 2 function 1 is the front door keypad front door function.
    //0ACC011100E6 <---Output change update 0A length, CC zone change message, 011 - Output number, 1 output state 1 on 0 off.
    //11KF02100000000009A <--- reply to key function.

    let ness_server = env::var("NESS_SERVER").unwrap();
    println!("Ness Server is {}", ness_server);

    use tiny_http::{Server, Response};
    let server = Server::http("0.0.0.0:8080").unwrap();
    for request in server.incoming_requests() {
        let result = match request.url() {
            "/gate" => {
                let command = "09kf02200".to_string();
                let result = generate_checksum(&command);
                let message = format!("{}{}\n\r", command, result);
                let tc = TelnetClient::new(ness_server.to_string(), 2101);
                let result = tc.send_msg(message);
                println!("Result is {:?}", result);
            },
            "/garage" => {
                let command = "09kf02300".to_string();
                let result = generate_checksum(&command);
                let message = format!("{}{}\n\r", command, result);
                let tc = TelnetClient::new(ness_server.to_string(), 2101);
                let result = tc.send_msg(message);
                println!("Result is {:?}", result);
            },
            "/door" => {
                let command = "09kf02100".to_string();
                let result = generate_checksum(&command);
                let message = format!("{}{}\n\r", command, result);
                let tc = TelnetClient::new(ness_server.to_string(), 2101);
                let result = tc.send_msg(message);
                println!("Result is {:?}", result);
            },
            _ => println!("Other")
        };

        let response = Response::from_string("Ness m1 - you should know the URLs");
	request.respond(response);
    }
}
