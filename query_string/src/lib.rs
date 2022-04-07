pub mod __uses {
    pub use form_urlencoded::Serializer as QuerySerializer;
    pub use serde::Serialize as SerdeSerialize;
    pub use serde_json::to_value as serde_json_to_value;
    pub use serde_json::Value as JsonValue;
}

pub use query_string_derive::QueryString;
