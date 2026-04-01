use std::fmt::Debug;

pub trait Speak {
    fn speak(&self) -> String;
    fn name(&self) -> String;
}

#[derive(Clone)]
pub struct Dog {
    pub name: String,
}

#[derive(Clone)]
pub struct Cat {
    pub name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Debug for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog {{ name: {} }}", self.name)
    }
}

pub fn announce(animal : impl Speak) {
    println!("Introducing: {}", animal.name());
    println!("{}", animal.speak());
}

pub fn loudest<T: Speak>(a: T, b:T) -> T{
    if a.speak().len() > b.speak().len() {
        a
    } else {
        b
    }
}

pub fn debug_announce<T>(animal: T)
where
    T: Speak + Debug,
{
    println!("Introducing: {:?}", animal);
    println!("{}", animal.speak());
}

pub fn pair_up<T,U>(first: T, second: U) 
where 
    T: Speak,
    U: Speak + Debug,
{
    println!("{} paired with {:?}", first.name(), second);
}