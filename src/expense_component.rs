use crate::{Category, Expense};
use std::fmt;

#[derive(Debug)]
pub struct ExpenseCategory {
    category: Category,
    subcategories: Vec<Expense>,
}

impl ExpenseCategory {
    pub fn new(category: Category) -> Self {
        ExpenseCategory {
            category,
            subcategories: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Expense) {
        self.subcategories.push(component);
    }
}

impl fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Category: {:?}", self.category)?;
        for subcategory in &self.subcategories {
            writeln!(f, "{}", subcategory)?;
        }

        Ok(())
    }
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "- {} | Amount: {} | Date: {:?}",
            self.description, self.amount, self.date
        )
    }
}
