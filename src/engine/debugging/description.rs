use ecs::ComponentTrait;

pub struct Description {
    pub name: String,
}

impl Description {
    pub fn new(name: &str) -> Description {
        Description {
            name: name.into(),
        }
    }
}

impl ComponentTrait for Description {}
