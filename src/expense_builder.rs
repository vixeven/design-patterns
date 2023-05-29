use crate::{Category, Expense};
use std::time::SystemTime;

pub struct ExpenseBuilder {
    id: u32,
    description: String,
    amount: f64,
    category: Category,
    date: Option<SystemTime>,
}

impl ExpenseBuilder {
    pub fn new(id: u32, description: &str, amount: f64, category: Category) -> Self {
        ExpenseBuilder {
            id,
            description: description.to_string(),
            amount,
            category,
            date: None,
        }
    }

    pub fn set_amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn set_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn set_date(mut self, date: SystemTime) -> Self {
        self.date = Some(date);
        self
    }

    pub fn build(self) -> Expense {
        Expense {
            id: self.id,
            description: self.description,
            amount: self.amount,
            category: self.category,
            date: self.date.unwrap_or_else(SystemTime::now),
        }
    }
}
