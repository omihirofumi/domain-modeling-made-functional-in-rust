// -----------------------------
// inputs to the workflow
struct UnvalidatedCustomerInfo {
    first_name: String,
    last_name: String,
    email_address: String,
}

struct UnvalidatedAddress {
    address_line1: String,
    address_line2: String,
    address_line3: String,
    address_line4: String,
    city: String,
    zip_code: String
}

struct UnvalidatedOrderLine {
    order_line_id: String,
    product_code: String,
    quantity: i32,
}

struct UnvalidatedOrder {
    order_id: String,
    customer_info: UnvalidatedCustomerInfo,
    shipping_address: UnvalidatedAddress,
    billing_address: UnvalidatedAddress,
    lines: Vec<UnvalidatedOrderLine>
}

// -----------------------------
// outputs from the workflow (success case)
// struct OrderAcknowledgmentSent {
//     order_id: OrderId,
//     email_address: EmailAddress,
// }
