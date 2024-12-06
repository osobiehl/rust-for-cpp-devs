struct Person{
    pub name: String,
    pub age: u32,
}

impl Person{
    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn age_to_statement(&self) -> &str {
        match self.age{
            0..=12 => "child",
            13..=19  => "teenager",
            _ => "adult"
        }
    }

    pub fn steal_name(self) -> String{
        self.name
    }
}
fn oldest_person(p1: &Person, p2: &Person) -> &Person{
    if p1.age > p2.age{
        return p1
    }
    else{
        return p2
    }
}



fn main() {

    let me = Person{name: "Jose".to_string(), age: 24};
    let you = Person{name: "Jacob".to_string(), age: 35};
    let oldest = oldest_person(&me, &you);
    println!("person is: a {}", oldest.age_to_statement());

    let your_name = you.steal_name();
    println!("I stole your name!: {your_name}");

    println!("you are {} years old", you.age);



}