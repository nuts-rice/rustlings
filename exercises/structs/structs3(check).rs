// structs3.rs
// Structs contain more than simply some data, they can also have logic, in this
// exercise we have defined the Package struct and we want to test some logic attached to it,
// make the code compile and the tests pass! If you have issues execute `rustlings hint structs3`

// I AM NOT DONE



//did the impl methods
//check it out

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            // Something goes here...
            return None;
        } else {
            return Package {sender_country, recipient_country, weight_in_grams};
        }
    }

    fn is_international(&self) -> bool {
        
        // Something goes here...
        self.sender_country != recipient_country

    }

    fn get_fees(&self, cents_per_kg: i32) -> i32 {
        // Something goes here... (beware of grams to kg conversion)
        let weight_in_kg = self.weight_in_grams * .001;
        let fee_in_cents = weight_in_kg * cents_per_kg;
        return fee_in_cents;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");
        
        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_kg = ???;
        
        let package = Package::new(sender_country, recipient_country, 1500);
        
        assert_eq!(package.get_fees(cents_per_kg), 4500);
    }
}