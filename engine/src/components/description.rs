use ecs::ComponentTrait;

#[derive(Clone)]
pub struct Description {
    pub name: String,
}

impl Description {
    pub fn new(name: &str) -> Self {
        Description {
            name: name.into(),
        }
    }
}

impl ComponentTrait for Description {}
