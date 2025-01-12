use serde::Deserialize;

use crate::orm::permissions;

#[derive(Deserialize, Debug)]
pub struct SetPermissionsRequest {
    pub permissions: permissions::Model,
}
