use sqlx::{sqlite::SqliteRow, FromRow, Row};

pub enum Permission {
    Moderate,
    Edit,
    Read,
    Denied,
}

impl Permission {
    pub fn can_read(&self) -> bool {
        !matches!(self, Self::Denied)
    }
    pub fn can_write(&self) -> bool {
        matches!(self, Self::Moderate | Self::Edit)
    }
}

impl<'r> FromRow<'r, SqliteRow> for Permission {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let permission = row.try_get("permission")?;
        Ok(match permission {
            "MODERATE" => Self::Moderate,
            "EDIT" => Self::Edit,
            "READ" => Self::Read,
            x => panic!("Bad permission {}", x),
        })
    }
}
