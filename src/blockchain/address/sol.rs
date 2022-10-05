use super::AddressValidation;
use anyhow::Result;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Sol<'a> {
    pub value: &'a str,
}

impl<'a> AddressValidation for Sol<'a> {}

impl<'a> Sol<'a> {
    pub fn new(address: &'a str) -> Result<Self> {
        let sol_address = Self { value: address };

        sol_address.is_valid_format()?;
        sol_address.is_valid_length()?;

        Ok(sol_address)
    }
}
