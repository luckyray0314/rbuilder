use alloy_primitives::{Address, U256};
use eyre::Result;
use crate::treasury::TreasuryConfig;

pub struct TreasuryConfig {
    pub address: Address,
    pub fee_percentage: u8,
}

impl TreasuryConfig {
    pub fn new(address: Address, fee_percentage: u8) -> Result<Self> {
        // Validate fee percentage is between 1-99
        if fee_percentage == 0 || fee_percentage >= 100 {
            return Err(eyre::eyre!("Treasury fee percentage must be between 1 and 99"));
        }

        Ok(Self {
            address,
            fee_percentage,
        })
    }

    pub fn calculate_split(&self, total_value: U256) -> (U256, U256) {
        let treasury_amount = (total_value * U256::from(self.fee_percentage)) / U256::from(100);
        let remaining = total_value - treasury_amount;
        (treasury_amount, remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_split_calculation() {
        let config = TreasuryConfig::new(
            Address::from_slice(&[0; 20]),
            10,
        ).unwrap();

        // Test with 100 ETH
        let total = U256::from(100_000_000_000_000_000_000u128); // 100 ETH in wei
        let (treasury, remaining) = config.calculate_split(total);
        
        assert_eq!(treasury, U256::from(10_000_000_000_000_000_000u128)); // 10 ETH
        assert_eq!(remaining, U256::from(90_000_000_000_000_000_000u128)); // 90 ETH
    }

    #[test]
    fn test_invalid_percentage() {
        assert!(TreasuryConfig::new(Address::from_slice(&[0; 20]), 0).is_err());
        assert!(TreasuryConfig::new(Address::from_slice(&[0; 20]), 100).is_err());
    }
}
