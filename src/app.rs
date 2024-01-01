use std::io;

use log::info;
use rib_server::message::Message;

pub fn run() -> io::Result<()> {
    let msg = Message::new("rib_server".to_string(), "Server started.".to_string());
    info!("{}", msg);
    Ok(())
}
