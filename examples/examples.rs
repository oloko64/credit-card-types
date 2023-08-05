use credit_card_types::{CreditCardPool, CreditCardType};

fn main() {
    // Create a new mutable pool, so we can add new card types to it.
    let mut pool = CreditCardPool::new();

    // Add a new card type to the pool.
    pool.insert_card_type(CreditCardType::default());

    // Get the credit card type for a given card number.
    let result = pool.get_credit_card_type("123456789").unwrap();

    // Print the result.
    println!("Credit card type: {:#?}", result);
}
