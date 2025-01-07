#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct UserAuthentication {}

impl UserAuthentication {
    pub fn is_valid(_username: &str, _password: &str) -> bool {
        todo!()
    }
}
