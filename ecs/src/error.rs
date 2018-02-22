use entity::Entity;

#[derive(Debug, Fail)]
pub enum EcsError {
    #[fail(display = "Could not find the specified component(s)")] ComponentNotFound,
    #[fail(display = "The specified component(s) were found more than once")]
    MultipleComponentsFound,
    #[fail(display = "The entity '{}' was not found in the assembly", _0)] EntityNotFound(Entity),
    #[fail(display = "The system's requirements were not satisfied")] UnsatisfiedRequirements,
}
