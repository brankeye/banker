pub struct Identity {
    first_name: String,
    last_name: String,
    age: u8,
    address: String,
}

impl Identity {
    pub fn new(first_name: String, last_name: String, age: u8, address: String) -> Self {
        Identity {
            first_name,
            last_name,
            age,
            address,
        }
    }

    fn print(&self) {
        println!(
            "Name: {} {}, Age: {}, Address: {}", 
            self.first_name, 
            self.last_name, 
            self.age, 
            self.address
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let jeff_identity = Identity::new(
            String::from("Jeff"), 
            String::from("Hanson"), 
            47, 
            String::from("123 Main St.")
        );
        jeff_identity.print();
    }
}
