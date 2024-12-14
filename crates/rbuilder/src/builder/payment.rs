use crate::treasury::TreasuryConfig;
use alloy_primitives::{Address, U256};

pub struct PaymentProcessor {
    treasury: Option<TreasuryConfig>,
}

impl PaymentProcessor {
    pub fn new(treasury: Option<TreasuryConfig>) -> Self {
        Self { treasury }
    }

    pub fn process_block_payment(&self, total_value: U256, recipient: Address) -> Vec<(Address, U256)> {
        let mut payments = Vec::new();

        match &self.treasury {
            Some(treasury) => {
                let (treasury_amount, remaining) = treasury.calculate_split(total_value);
                
                // Add treasury payment if non-zero
                if !treasury_amount.is_zero() {
                    payments.push((treasury.address, treasury_amount));
                }
                
                // Add remaining payment to recipient if non-zero
                if !remaining.is_zero() {
                    payments.push((recipient, remaining));
                }
            }
            None => {
                // No treasury configured, send full amount to recipient
                if !total_value.is_zero() {
                    payments.push((recipient, total_value));
                }
            }
        }

        payments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_processing_with_treasury() {
        let treasury_addr = Address::from_slice(&[1; 20]);
        let treasury = TreasuryConfig::new(treasury_addr, 10).unwrap();
        let processor = PaymentProcessor::new(Some(treasury));

        let recipient = Address::from_slice(&[2; 20]);
        let total_value = U256::from(100_000_000_000_000_000_000u128); // 100 ETH

        let payments = processor.process_block_payment(total_value, recipient);
        
        assert_eq!(payments.len(), 2);
        assert_eq!(payments[0].0, treasury_addr);
        assert_eq!(payments[0].1, U256::from(10_000_000_000_000_000_000u128)); // 10 ETH
        assert_eq!(payments[1].0, recipient);
        assert_eq!(payments[1].1, U256::from(90_000_000_000_000_000_000u128)); // 90 ETH
    }

    #[test]
    fn test_payment_processing_without_treasury() {
        let processor = PaymentProcessor::new(None);
        let recipient = Address::from_slice(&[2; 20]);
        let total_value = U256::from(100_000_000_000_000_000_000u128); // 100 ETH

        let payments = processor.process_block_payment(total_value, recipient);
        
        assert_eq!(payments.len(), 1);
        assert_eq!(payments[0].0, recipient);
        assert_eq!(payments[0].1, total_value);
    }
}