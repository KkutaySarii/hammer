use std::io;

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub state: String,
    pub population: i64,
    pub user_number: i64,
}

pub struct Node {
    pub city: City,
    pub next: Option<Box<Node>>,
}

pub struct LinkedList {
    pub head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.city);
            current = &node.next;
        }
    }
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn add(&mut self, city: City) {
        if self.check_add(city.name.clone()) {
            let mut new_node = Box::new(Node::new(city));
            new_node.next = self.head.take();
            self.head = Some(new_node);
            println!("Added successfully!\n");
        } else {
            println!("This city added before!\n");
        }
    }

    pub fn search(&self, name: String) -> Option<&City> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.city.name == name {
                return Some(&node.city);
            }
            current = &node.next;
        }
        None
    }

    pub fn check_add(&self, name: String) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.city.name == name {
                return false;
            }
            current = &node.next;
        }
        true
    }
    pub fn delete(&mut self, name: String) {
        if self.head.is_none() {
            println!("The list is empty!");
            return;
        }
        if self.head.as_ref().unwrap().city.name == name {
            self.head = self.head.take().unwrap().next;
            return;
        }
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.city.name == name {
                    node.next = next_node.next.take();
                    return;
                }
            }
            current = &mut node.next;
        }
    }
    pub fn delete_last(&mut self) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                current = &mut None;
                break;
            }
            current = &mut node.next;
        }
    }
    pub fn update_population(&mut self, name: String, new_population: i64) -> Option<&City> {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.city.name == name {
                node.city.population = new_population;
                return Some(&node.city);
            }
            current = &mut node.next;
        }
        None
    }
    pub fn update_user_number(&mut self, name: String, new_user_number: i64) -> Option<&City> {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.city.name == name {
                node.city.user_number = new_user_number;
                return Some(&node.city);
            }
            current = &mut node.next;
        }
        None
    }
}

impl Node {
    pub fn new(city: City) -> Self {
        Node { city, next: None }
    }
}

impl City {
    pub fn new(name: String, state: String, population: i64, user_number: i64) -> Self {
        City {
            name,
            state,
            population,
            user_number,
        }
    }
}
impl std::fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "name : {}state: {}population: {},\nuser number : {}\n",
            self.name, self.state, self.population, self.user_number
        )
    }
}

pub fn take_input(message: &str) -> String {
    println!("{message}");
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line");
    data
}
