use std::ops::{Add, BitAnd, BitOrAssign};

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "permission_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(primary_key)]
    pub user: i32,
    pub permissions: u64,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

pub enum Permission {
    BookAdd = 0b1,
    BookUpdate = 0b10,
    BookDelete = 0b100,

    UserAdd = 0b1000,
    UserUpdate = 0b10000,
    UserDelete = 0b100000,

    PermissionsUpdate = 0b1000000,
}

impl BitAnd<Permission> for Model {
    type Output = bool;
    fn bitand(self, rhs: Permission) -> Self::Output {
        (self.permissions & (rhs as u64)) != 0
    }
}

impl Add<Permission> for Model {
    type Output = Model;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(mut self, rhs: Permission) -> Self::Output {
        self.permissions |= rhs as u64;
        self
    }
}

impl BitOrAssign<Permission> for Model {
    fn bitor_assign(&mut self, rhs: Permission) {
        self.permissions |= rhs as u64;
    }
}
