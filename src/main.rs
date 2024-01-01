mod app;

use env_logger;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().parse_env("RIB_LOG").init();

    match app::run() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
