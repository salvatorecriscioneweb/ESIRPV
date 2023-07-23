pub struct OrderProduct {
    quantity: usize,
    sku: String,
}
pub struct InfoAddress {
    city: String,
    cap: String,
    name: String,
    surname: String,
    phone: String,
}
pub struct Order {
    id: String,                          // mandatory
    number: String,                      // mandatory
    products: Option<Vec<OrderProduct>>, // can be empty
    receiver: InfoAddress,               // mandatory
    sender: InfoAddress,                 // mandatory
    timestamp: String,                   // mandatory
}

impl Order {}
