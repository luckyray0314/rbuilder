# Fee Withholding Feature

The rbuilder supports withholding a configurable percentage of block rewards to a treasury address. By default, 10% of the total block value is withheld.

## Configuration

Add the following to your config file (e.g. `config.toml`):

```toml
[fee_distribution]
treasury_address = "0x..." # The Ethereum address that will receive the withheld fees
withholding_percentage = 10 # Percentage to withhold (0-100)
```

## How it Works

When a block is built and rewards are calculated:

1. The total block value is calculated (block rewards + tips + MEV)
2. The configured percentage (default 10%) is withheld for the treasury
3. The remaining amount (default 90%) is sent to the intended recipient

## Testing Locally

1. Configure your treasury address in the config file
2. Run rbuilder normally
3. Monitor the treasury address for incoming fees
4. Use the integration tests to verify the distribution logic

## Example Calculation

For a block worth 100 ETH:
- 10 ETH goes to the treasury address
- 90 ETH goes to the intended recipient

## Implementation Details

The fee withholding happens at the final payment stage, after all other calculations are complete. This ensures that the withholding is based on the true final value of the block.