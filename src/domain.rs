struct WidgetCode(String);

struct GizmoCode(String);

enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

struct UnitQuantity(u32);

struct KilogramQuantity(f32);

enum OrderQuantity {
    Unit(UnitQuantity),
    Kilos(KilogramQuantity),
}

struct OrderId;

struct OrderLineId;

struct CustomerId;

struct CustomerInfo;

struct ShippingAddress;

struct BillingAddress;

struct Price;

struct BillingAmount;

struct Order {
    id: OrderId,
    customer_id: CustomerId,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: Vec<OrderLine>,
    ammount_to_bill: BillingAmount,
}

struct OrderLine {
    id: OrderLineId,
    order_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

struct UnvalidatedOrder {
    order_id: String,
    customer_info: String,
    shipping_address: String,
    // ...
}
