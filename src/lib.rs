//! Helper code to perform HMAC-SHA256 using Cryptographic Circuits.
//! Evaulator Example -> ./garbled -e -v -i 0xb598163d740b0973b8b312881bfe6601e031d33d9f15be97b0cc8898ae570932fd755ced0af309f7625f531ab01cdc0ba7130ae14b561b905f53777255174170 examples/rapid-hmac.mpcl
//! Garbler Example -> ./garbled -v -i 0x48656c6c6f2c20776f726c64212e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2e2e,0x43 examples/rapid-hmac.mpcl
//! Run using ```cargo watch -q -c -w src/ -x run```

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::env;
use std::process::{Command, exit, Child};
use std::thread;
use std::time::Duration;

// #[derive(Deserialize, Debug,Serialize)]
// pub struct HMACResponse{
//     pub signature: String,
// }

// #[derive(Deserialize, Debug, Serialize)]
// pub struct HMACError{
//     pub message: String
// }

pub async fn hmac_sha256(
    key: String,
    message: String
)-> String {
// Helper code to define path for garbler command line program.
    let mut path = String::new();

    if let Ok(current_dir) = env::current_dir() {
        let current_dir_string = current_dir.to_string_lossy().into_owned();
        path = current_dir_string;
    } else {
        eprintln!("Error getting current directory");
    }

//  Key -> The HMAC-SHA256 Secret Key. Message -> Data to be signed with Secret.
    let garbler_path = path.to_string() + "/src";
    let garbler_path = "/Users/soms/Development/rapid_hmac/src".to_string();

    let evaluator_node = format!("./garbled -e -v -i {key} examples/rapid-hmac.mpcl");
    let garbler_node = format!("./garbled -v -i {message},0x00 examples/rapid-hmac.mpcl");
    
    let mut evaluator_node = Command::new("bash")
         .arg("-c")
         .arg(format!("cd {} && {}", garbler_path, evaluator_node))
         .spawn()
         .expect("failed to execute process");

    thread::sleep(Duration::from_secs(2));

    let output1 = Command::new("bash")
        .arg("-c")
        .arg(format!("cd {} && {}",garbler_path, garbler_node))
        .output()
        .expect("failed to execute process");
    
    let output1 = String::from_utf8_lossy(&output1.stdout) ;

    if let Some(result) = output1.lines().find(|line| line.starts_with("Result[0]: ")) {
        let hmac_sign_result = &result[11..];
        let sign = String::from(hmac_sign_result);
        return sign
    } else {
        let error = String::from("Result not found in output");
        return error

    }
}
