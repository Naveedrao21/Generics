
             trait Greet {
                fn greet(&self) -> String;
            }
            struct Person {
                name: String,
            } 
            
            impl Greet for Person {
                fn greet(&self) -> String{
                    format!("Hi i am {}", self.name)
                }
            }
            fn main () {
                let p = Person {
                    name: String::from("Naveed"),
                   
                };
             println!("{}" ,p.greet());
            }