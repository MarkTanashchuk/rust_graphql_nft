mod eth;
mod sol;

use anyhow::Result;

pub use eth::Eth;
pub use sol::Sol;

pub(self) trait AddressValidation {
    fn is_valid_length(&self) -> Result<()> {
        unimplemented!("Unsupported address format");
    }

    fn is_valid_format(&self) -> Result<()> {
        unimplemented!("Unsupported address format");
    }
}
