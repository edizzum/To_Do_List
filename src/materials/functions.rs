use crate::materials::structs::Item;
use std::{io};

pub fn add_item(list: &mut Vec<Item>){
    let mut newdata = String::new();
    io::stdin().read_line(&mut newdata).expect("Failed to read line");
    let x = Item{data: newdata, completed: false};
    println!("Item {} added! What else you want to do?", x.data);
    list.push(x);
}

pub fn delete_item(list: &mut Vec<Item>){
    todo_list(list);
    let mut numberinput = String::new();
    io::stdin().read_line(&mut numberinput).expect("Failed to read line");
    let datas: usize = numberinput.trim().parse().expect("Please enter a valid number");
    println!("Index of Item: {} removed! What else you want to do", datas);
    list.remove(datas);
}

pub fn complete_item(completed: &mut Vec<Item>, array: &mut Vec<Item>){
    let mut newd: String = String::new();
    io::stdin().read_line(&mut newd).expect("Failed to read line");
    let x: usize = newd.trim().parse().expect("Please enter a valid number");
    let z = array.remove(x);
    let compl = Item{data: z.data, completed: true};
    completed.push(compl);
}

pub fn todo_list(list: &mut Vec<Item>){
    for item in list {
        println!("Data:{} Completed:{}\n", item.data, item.completed);
    }
}

pub fn completed_items(completed: &mut Vec<Item>) {
    for item in completed {
        println!("Data:{} Completed:{}\n", item.data, item.completed);
    }
}