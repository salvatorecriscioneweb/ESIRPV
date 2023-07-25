// ProductsList in Django
pub struct ProductsList {
    order: String,
    quantity: usize,
    product: String,
}
pub struct Client {
    id: String,
    name: String,
    surname: String,
    address: String,
}
pub struct Order {
    order_id: String,                   // mandatory
    number: String,                     // mandatory
    product: Option<Vec<ProductsList>>, // can be empty
    client: String,                     // mandatory client
    timestamp: String,                  // mandatory
}

impl Order {}
impl ProductsList {}
impl Client {}
