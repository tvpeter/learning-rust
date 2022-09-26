/*
    Topic: Generics and Functions

    Requirements:
    - create a fn that accepts the Priority trait as a generic parameter
        - the fn should print out the guest and their priority
    - Use the fn with at least two different guests

    Notes:
    - use the debug token {:?} to print out the information
    - use the compiler to guide you to the correct generic constraints needed
*/

#[derive(Debug, Clone, Copy)]
enum ServicePriority {
    High,
    Standard
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest {
    name: String,
    priority: ServicePriority,
}

impl ImportantGuest {
    fn new(name: String) -> Self {
        Self { name, priority: ServicePriority::High }
    }
}

impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        self.priority
    }
}
#[derive(Debug)]
struct Guest {
    name: String,
    priority: ServicePriority,
}

impl Guest {
    fn new(name: String) -> Self {
        Self { name, priority: ServicePriority::Standard }
    }
}

impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        self.priority
    }
}

fn guest_priority<T: Priority + std::fmt::Debug>(user: T) {
    println!(" {:?} has prioty: {:?}", user, user.get_priority());
}
fn main(){
    let std_guest = Guest::new("Peter".to_owned());
    let imp_guest = ImportantGuest::new("Theo".to_owned());

    guest_priority(std_guest);
    guest_priority(imp_guest);
}
