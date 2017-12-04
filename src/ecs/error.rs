#[derive(Debug, Fail)]
pub enum EcsError {
    #[fail(display = "Could not find the components '{}'", _0)]
    ComponentNotFound(String),
    #[fail(display = "The components '{}' were found more than once", _0)]
    MultipleComponentsFound(String),
}
