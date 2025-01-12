use serde::Serialize;

use crate::orm::permissions;

#[derive(Serialize, Debug)]
pub struct GetPermissionsResponse {
    pub permissions: permissions::Model,
}
