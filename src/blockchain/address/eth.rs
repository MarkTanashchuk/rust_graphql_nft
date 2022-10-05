use super::AddressValidation;
use anyhow::{anyhow, Result};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Eth<'a> {
    pub value: &'a str,
}

impl<'a> AddressValidation for Eth<'a> {
    fn is_valid_format(&self) -> Result<()> {
        let chars = self.value.chars().collect::<Vec<char>>();

        if let Some(&['0', 'x']) = chars.get(0..=1) {
            Ok(())
        } else {
            Err(anyhow!("Invalid format"))
        }
    }

    fn is_valid_length(&self) -> Result<()> {
        if self.value.len() == 42 {
            Ok(())
        } else {
            Err(anyhow!("Invalid length"))
        }
    }
}

impl<'a> Eth<'a> {
    pub fn new(address: &'a str) -> Result<Self> {
        let eth_address = Self { value: address };

        eth_address.is_valid_format()?;
        eth_address.is_valid_length()?;

        Ok(eth_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_length() {
        assert!(Eth::new("0x00").is_err());
    }

    #[test]
    fn invalid_format() {
        assert!(Eth::new("XXXX").is_err());
    }

    #[test]
    fn valid_value() {
        let value = "0x0000000000000000000000000000000000000000";
        assert!(matches!(
            Eth::new(value),
            Ok(Eth {
                value: "0x0000000000000000000000000000000000000000"
            })
        ));
    }
}
