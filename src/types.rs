pub struct String50(String);
pub struct EmailAddress(String);
pub struct ZipCode(String);
pub struct OrderId(String);
pub struct OrderLineId(String);
pub struct WidgetCode(String);
pub struct GizmoCode(String);
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}
pub struct UnitQuantity(u16);
pub struct KilogramQuantity(f32);
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}
pub struct Price(f64);
pub struct BillingAmount(f64);
pub struct PdfAttachment {
    name: String,
    bytes: [u8],
}

#[derive(Debug)]
pub struct ValueError(String);

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ValueError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::error::Error for ValueError {}

impl String50 {
    pub fn new(value: String) -> Result<String50, ValueError>  {
        if value.is_empty() {
            return Err(ValueError("value must not be empty".to_string()));
        }
        Ok(String50(value))
    }
}
