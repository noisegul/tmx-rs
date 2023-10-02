pub struct Property {
    name: String,
    property_type: String,
    value: String, // normally: PropertyValue
}

/// Since the type of the property can be (quote:) string, int, float or
/// bool, we use an enum for the property_value.
/// TODO: `property_type` is currently declared as a String, not sure if
/// this field is needed at all.
enum PropertyValue {
    String,
    I32,
    F32,
    Bool
}
