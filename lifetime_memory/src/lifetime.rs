struct Person {
    name: String
}

impl Person {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        return &self.name;
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

fn main() {
    let boss = Person{name: String::from("Tom")};
    let kfc = Company{name: String::from("KFC"), ceo: &boss};

    let mut z:&String;
    {
        let p = Person{name: String::from("John")};
        z = p.get_ref_name();
        println!("name = {}", z);
    }
}