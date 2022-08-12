mod other;

use eyre::{Result, eyre};

#[allow(dead_code)]
fn foo() -> Result<()> {
    Err(eyre!("this is an error"))
}
