use std::thread;
use std::time::Duration;

pub trait Metadata {
    fn common(&self) -> String;
    fn specific(&self) -> String; 
}

pub fn sleep() {
    let secs: u64 = 1;
    thread::sleep(Duration::from_secs(secs));
}