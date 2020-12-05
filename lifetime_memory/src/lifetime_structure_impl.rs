struct Person<'a> {
    name: &'a str
}

impl<'b> Person<'b> {
    fn talk(&self) {
        println!("Hi my name is {}.", self.name);
    }
}

fn main() {
    let person = Person{name: "Tom"};
    person.talk()
}