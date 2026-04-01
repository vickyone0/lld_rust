pub trait Describe{

    fn name(&self) -> String;

    fn describe(&self) -> String{
        format!("hello, {}", self.name())
    }

    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}


pub struct Dog {
    pub name: String,
}

impl Describe for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Robot {
    pub id: u32,
}

impl Describe for Robot {
    fn name(&self) -> String {
        format!("Robot {}", self.id)
    }

     fn describe(&self) -> String {
        format!("UNIT {} ONLINE", self.id)
    }
}