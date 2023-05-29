mod caching_expense_manager;
mod command;
mod expense_builder;
mod expense_component;
mod expense_factory;
mod expense_manager;
mod exporter;
mod observer;

use command::AddExpenseCommand;
use command::Command;
use expense_factory::ExpenseFactory;
use expense_manager::ExpenseManager;
use std::time::SystemTime;

use crate::caching_expense_manager::CachingExpenseManager;
use crate::command::DeleteExpenseCommand;
use crate::command::EditExpenseCommand;
use crate::expense_builder::ExpenseBuilder;
use crate::expense_component::ExpenseCategory;
use exporter::{CsvExporter, Exporter, JsonExporter};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Category {
    Food,
    Travel,
    Utilities,
    Entertainment,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: f64,
    pub category: Category,
    pub date: SystemTime,
}

fn main() {
    /* Singleton pattern */
    let mut manager = ExpenseManager::new();

    /* Factory Method pattern */
    let food_expense = Category::Food.create_expense(1, "Groceries", 50.0, SystemTime::now());
    let travel_expense = Category::Travel.create_expense(2, "Bus ticket", 2.5, SystemTime::now());
    let utilities_expense =
        Category::Utilities.create_expense(3, "Bills", 100.0, SystemTime::now());

    /* Command pattern */
    let add_food_expense = AddExpenseCommand::new(food_expense);
    let add_travel_expense = AddExpenseCommand::new(travel_expense);
    let add_utilities_expense = AddExpenseCommand::new(utilities_expense);

    add_food_expense.execute(&mut manager);
    add_travel_expense.execute(&mut manager);
    add_utilities_expense.execute(&mut manager);

    println!("-- Expenses: {}", manager);

    let edited_expense =
        Category::Food.create_expense(1, "Groceries (edited)", 55.0, SystemTime::now());
    let edit_expense = EditExpenseCommand::new(1, edited_expense);
    edit_expense.execute(&mut manager);

    let delete_expense = DeleteExpenseCommand::new(2);
    delete_expense.execute(&mut manager);

    println!("-- Expenses after editing and deleting: {}", manager);

    let expenses = manager.get_expenses();

    /* Adapter pattern */
    let csv_exporter = CsvExporter;
    let json_exporter = JsonExporter;

    match csv_exporter.export(&expenses) {
        Ok(csv_data) => println!(" -- CSV data:\n{}", csv_data),
        Err(error) => println!("Error exporting to CSV: {}", error),
    }

    match json_exporter.export(&expenses) {
        Ok(json_data) => println!(" -- JSON data:\n{}", json_data),
        Err(error) => println!("Error exporting to JSON: {}", error),
    }

    /* Composite pattern */
    let expenses = manager.get_expenses();

    let mut food_category = ExpenseCategory::new(Category::Food);
    let mut utilities_category = ExpenseCategory::new(Category::Utilities);

    for expense in expenses {
        match expense.category {
            Category::Food => food_category.add(expense.clone()),
            Category::Utilities => utilities_category.add(expense.clone()),
            _ => (),
        }
    }

    println!("\n-- Expenses by category:");
    println!("{}", food_category);
    println!("{}", utilities_category);

    /* Builder Pattern */
    let entertainment_expense =
        ExpenseBuilder::new(3, "Movie ticket", 10.0, Category::Entertainment)
            .set_date(SystemTime::now())
            .set_category(Category::Entertainment)
            .set_description("Movie ticket for Avengers: Endgame")
            .set_amount(15.0)
            .build();

    /* Proxy pattern */
    let mut caching_manager = CachingExpenseManager::new(manager.clone());

    println!(
        "-- Total expenses: {}",
        caching_manager.calculate_total_expenses()
    );

    /* Observer pattern */
    manager
        .events()
        .subscribe(observer::Event::Add, caching_manager.subscriber);

    caching_manager.add_expense(entertainment_expense);

    manager
        .events()
        .unsubscribe(observer::Event::Add, caching_manager.subscriber);

    println!(
        "-- Total expenses after update: {}",
        caching_manager.calculate_total_expenses()
    );

    /* Iterator pattern */
    println!("-- Iterator pattern:");
    for expense in caching_manager.manager {
        println!("{:?}", expense);
    }
}
