/**
 * Topic: Result and the question mark operator
 * 
 * Requirements:
 * Determine if an employee can access a building using a digital keycard 
 * Employees that can access the building are:
 * - Maintenance crews
 * - Marketing department employees
 * - Managers
 * Other employees that work at the company are:
 * Line Supervisors
 * Kitchen staff
 * Assemply technicians
 * Ensure that terminated employees cannot access the building regardless of their position
 * 
 * Notes:
 * use an enum to represent all types of employees
 * use a struct to store the employee type and whether they are still employed
 * use a function that returns a Result to determine if the employee may enter the building
 * print whether the employee may access the building 
 * must use a function that utilizes the question mark to do this
 */

 enum Role {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Technician,
 }

 #[derive(PartialEq)]
 enum Status {
    Active,
    Terminated,
 }

 struct Employee {
    role: Role,
    status: Status,
 }

 fn check_access(employee: &Employee) -> Result<(), String> {
    if employee.status == Status::Terminated { 
        return Err("Employee cannot access the building".to_owned()) 
    }
    
    match employee.role {
        Role::Maintenance => Ok(()),
        Role::Marketing => Ok(()),
        Role::Manager => Ok(()),
        _ => Err("Employee cannot access the building".to_owned()),
    }
 }

 fn print_status(employee: &Employee) -> Result<(), String> {
    check_access(employee)?;
    Ok(())
 }
fn main(){
    let employee = Employee {
        role: Role::Manager,
        status: Status::Active,
    };

    match print_status(&employee) {
        Err(e) => println!("Error: {:?}", e),
        _ => println!("access granted"),
    }
    
}
