use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// <https://docs.curseforge.com/#tocS_CoreStatus>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_CoreApiStatus>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
    #[cfg(feature = "allow-unknown-fields")]
    #[serde(other)]
    Unknown = u8::MAX,
}

/// <https://docs.curseforge.com/#tocS_Pagination>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pagination {
    pub index: i32,
    pub page_size: i32,
    pub result_count: i32,
    pub total_count: i64,
}
