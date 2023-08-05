# Credit Card Types

The Credit Card Type library for Rust provides a utility method to determine the type of credit card based on both fully qualified and partial card numbers.

## Features

- Determine the type of credit card based on the provided card number (fully qualified or partial).
- Support for popular credit card types like Visa, MasterCard, American Express, Discover, and more.
- No dependencies.

## Usage

```rust
use credit_card_types::{CreditCardPool, CreditCardType};

fn main() {
    // Create a new pool.
    let pool = CreditCardPool::new();

    // Get the credit card type for a given card number.
    let result = pool.get_credit_card_type("4111111111111111").unwrap();

    // Print the result.
    println!("Credit card type: {:#?}", result);
}
```

#### This crate was inspired by the [credit-card-type](https://github.com/braintree/credit-card-type) NPM package.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
