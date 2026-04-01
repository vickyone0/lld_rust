pub trait Speak {
    fn speak(&self) -> String;
    fn name(&self) -> String;
    fn introduce(&self) -> String {
        format!("My name is {} and saying {}", self.name(), self.speak())   
    }
}


pub struct Dog{
    pub name: String,
}
pub struct Cat{
    pub name: String,
}

pub struct Robot {
    pub id: u32,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("Woof! My name is {}", self.name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("Meow! My name is {}", self.name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Speak for Robot {
    fn speak(&self) -> String {
        format!("Beep boop! My ID is {}", self.id)
    }

    fn name(&self) -> String {
        format!("Robot {}", self.id)
    }
}

