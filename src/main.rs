mod linked_cities;
use linked_cities::{City, LinkedList};
fn main() {
    let mut linked_cities = LinkedList::new();
    println!(
        "1-Add City\n2-Delete City\n3-Delete Last City\n4-Print Cities\n5-Search City\n6-Update Population\n7-Update User Number\n8-Exit\n"
    );
    loop {
        let select = linked_cities::take_input("Please select the action you want to do: ");
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match select {
            1 => {
                let name = linked_cities::take_input("Name: ");
                let state = linked_cities::take_input("State: ");
                let population = linked_cities::take_input("Population: ");
                let population: i64 = match population.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let user_number = linked_cities::take_input("User number: ");
                let user_number: i64 = match user_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let city = City::new(name, state, population, user_number);
                linked_cities.add(city);
            }
            2 => {
                let name = linked_cities::take_input("Name: ");
                linked_cities.delete(name);
            }
            3 => linked_cities.delete_last(),
            4 => linked_cities.display(),
            5 => {
                let name = linked_cities::take_input("Name");
                let city = linked_cities.search(name);
                match city {
                    Some(c) => println!("City found:\n{}", c),
                    None => println!("City not found"),
                }
            }
            6 => {
                let name = linked_cities::take_input("Name");
                let new_population = linked_cities::take_input("New population: ");
                let new_population: i64 = match new_population.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                linked_cities.update_population(name, new_population);
            }
            7 => {
                let name = linked_cities::take_input("Name");
                let new_user_number = linked_cities::take_input("New User Number: ");
                let new_user_number: i64 = match new_user_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                linked_cities.update_user_number(name, new_user_number);
            }
            8 => {
                println!("You left the menu");
                break;
            }
            _ => println!("Wrong choice, try again"),
        }
    }
}
