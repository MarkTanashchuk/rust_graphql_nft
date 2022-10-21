use anyhow::{anyhow, Result};

/// Structure that holds validated Ethereum address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressEth {
    /// Address string
    pub inner: String,
}

impl AddressEth {
    /// Creates a validated instance of the Ethereum address
    ///
    /// # Errors
    /// Returns `Err` if the length of `address` is not equal to 42 and does not start with `0x`
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use blockchain::AddressEth;
    ///
    /// let address = "0x0000000000000000000000000000000000000000";
    /// let address = AddressEth::new(address.to_string());
    /// ```
    pub fn new(address: String) -> Result<Self> {
        if address.len() != 42 {
            return Err(anyhow!("Invalid Ethereum address: Invalid length"));
        }

        if !address.starts_with("0x") {
            return Err(anyhow!("Invalid Ethereum address: Invalid format"));
        }

        Ok(Self { inner: address })
    }
}
