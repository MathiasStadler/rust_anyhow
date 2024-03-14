#[allow(unused_imports)]
use anyhow::{Context, Result};
#[allow(unused_imports)]
use std::error::Error;
#[allow(unused_imports)]
use std::fmt::{Debug, Display, Formatter};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("ERROR: {:#?}", err);
        err.chain()
            .skip(1)
            .try_for_each(|cause| -> Result<(), Box<dyn Error + Send + Sync>> {
                eprintln!("because: {}", cause);
                Ok(())
            }
            }

        std::process::exit(1);
    
    
     std::process::exit(0);
}

fn try_main() {
    println!("Hello anyhow");
}
