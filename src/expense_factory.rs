use crate::{Category, Expense};
use std::time::SystemTime;

pub trait ExpenseFactory {
    fn create_expense(&self, id: u32, description: &str, amount: f64, date: SystemTime) -> Expense;
}

impl ExpenseFactory for Category {
    fn create_expense(&self, id: u32, description: &str, amount: f64, date: SystemTime) -> Expense {
        Expense {
            id,
            description: description.to_string(),
            amount,
            category: self.clone(),
            date,
        }
    }
}
