use std::error::Error;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use duplicate::duplicate_item;
use postgres_from_row::FromRow;
use postgres_types::private::BytesMut;
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenCode {
    A,
    B,
    C,
    D,
    E,
}

impl Display for TokenCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
        }
    }
}

impl FromSql<'_> for TokenCode {
    fn from_sql(_ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match raw {
            b"A" => Ok(Self::A),
            b"B" => Ok(Self::B),
            b"C" => Ok(Self::C),
            b"D" => Ok(Self::D),
            _ => Ok(Self::E),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusChoice {
    Pending,
    Success,
    Fail,
    Timeout,
}

impl Display for StatusChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Success => write!(f, "success"),
            Self::Fail => write!(f, "fail"),
            Self::Timeout => write!(f, "timeout"),
        }
    }
}

impl FromSql<'_> for StatusChoice {
    fn from_sql(_ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match raw {
            b"pending" => Ok(Self::Pending),
            b"success" => Ok(Self::Success),
            b"fail" => Ok(Self::Fail),
            _ => Ok(Self::Timeout),
        }
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}

#[duplicate_item(type_name;[TokenCode];[StatusChoice])]
impl ToSql for type_name {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized + Display,
    {
        format!("{}", self).to_sql(ty, out)
    }

    fn accepts(_ty: &Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    to_sql_checked!();
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: StatusChoice,
    pub from_user_id: String,
    pub to_user_id: String,
    pub order_id: String,
    pub point: i64,
    pub coin_code: TokenCode,
    pub ext_json: Option<String>,
    pub gen_time: i64,
    pub tx_hash: Option<String>,
    pub tag_id: String,
    pub fail_reason: Option<String>,
    pub status_code: i32,
    pub store_id: Option<String>,
    pub block_number: i64,
    pub success_time: Option<DateTime<Utc>>,
    pub request_time: Option<DateTime<Utc>>,
}
