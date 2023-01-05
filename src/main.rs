pub mod materials;
use crate::materials::Functions::{self, add_item, delete_item, complete_item, completed_items, todo_list};
use crate::materials::Structs::Item;
use std::io;

fn main() {
    let mut array: Vec<Item> = Vec::new();
    let mut completeditems: Vec<Item> = Vec::new();
    loop{
        println!("---TO DO APP---\n
        Please select which option you want to do:
        1-Add Item\n
        2-Delete Item\n
        3-Complete Item\n
        4-See To Do List\n
        5-See Completed Items\n
        6-Exit\n");
        let mut select = String::new();
        println!("Please select the action you want to do: ");
        io::stdin().read_line(&mut select).expect("Failed to read line");
        let select: i8 = select.trim().parse().expect("Please enter a valid number");
        match select {
            1 => {
                println!("Please write the to do name which you want to add to the list: ");
                add_item(&mut array)},
            2 => {
                println!("Please write the index which you want to remove: ");
                delete_item(&mut array)},
            3 => {
                println!("Please write the index which you want to mark as completed: ");
                complete_item(&mut completeditems);
                delete_item(&mut completeditems)},
            4 => {
                println!("To do list: ");
                todo_list(&mut array)},
            5 => {
                println!("Completed items: ");
                completed_items(&mut completeditems)},
            6 => {
                println!("Program closed! See you again :)");
                break;
            }
            _ => println!("Wrong choice"),
        }
    }  
}