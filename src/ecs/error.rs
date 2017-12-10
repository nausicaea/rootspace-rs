use super::entity::Entity;

#[derive(Debug, Fail)]
pub enum EcsError {
    #[fail(display = "Could not find the components '{}'", _0)]
    ComponentNotFound(String),
    #[fail(display = "The components '{}' were found more than once", _0)]
    MultipleComponentsFound(String),
    #[fail(display = "The entity '{}' was not found in the assembly", _0)]
    EntityNotFound(Entity),
}
