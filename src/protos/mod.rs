pub mod convert;
pub mod display;
pub mod spotware_message {
    include!(concat!(env!("OUT_DIR"), "/spotware-message.rs"));
}
