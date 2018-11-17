#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Milliunits(i64);

impl Milliunits {
    pub fn new(val: i64) -> Self {
        Milliunits(val)
    }
    pub fn to_int(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Display, EnumIter, EnumString, Copy, Clone, PartialEq, Eq)]
pub enum TransactionType {
    #[strum(serialize = "uncategorized")]
    Uncategorized,
    #[strum(serialize = "unapproved")]
    Unapproved,
}

#[derive(Debug, Display, EnumIter, EnumString, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Cleared {
    #[strum(serialize = "cleared")]
    #[serde(rename = "cleared")]
    Cleared,
    #[strum(serialize = "uncleared")]
    #[serde(rename = "uncleared")]
    Uncleared,
    #[strum(serialize = "reconciled")]
    #[serde(rename = "reconciled")]
    Reconciled,
}

#[derive(Debug, Display, EnumIter, EnumString, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FlagColor {
    #[strum(serialize = "red")]
    #[serde(rename = "red")]
    Red,
    #[strum(serialize = "orange")]
    #[serde(rename = "orange")]
    Orange,
    #[strum(serialize = "yellow")]
    #[serde(rename = "yellow")]
    Yellow,
    #[strum(serialize = "green")]
    #[serde(rename = "green")]
    Green,
    #[strum(serialize = "blue")]
    #[serde(rename = "blue")]
    Blue,
    #[strum(serialize = "purple")]
    #[serde(rename = "purple")]
    Purple,
}

#[derive(Debug, Display, EnumIter, EnumString, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HybridTransactionType {
    #[strum(serialize = "transaction")]
    #[serde(rename = "transaction")]
    Transaction,
    #[strum(serialize = "subtransaction")]
    #[serde(rename = "subtransaction")]
    Subtransaction,
}
