use std::fmt;

use crate::{
    observer::{Event, Publisher},
    Expense,
};

#[derive(Debug, Clone)]
pub struct PartialExpense {
    pub description: String,
    pub amount: f64,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct ExpenseManager {
    expenses: Vec<Expense>,
    publisher: Publisher,
}

impl ExpenseManager {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn new() -> Self {
        ExpenseManager {
            expenses: Vec::new(),
            publisher: Publisher::default(),
        }
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.publisher.notify(Event::Add, expense.clone());
        self.expenses.push(expense);
    }

    pub fn get_expenses(&self) -> Vec<Expense> {
        self.expenses.clone()
    }

    pub fn edit_expense(&mut self, id: u32, new_expense: Expense) {
        self.publisher.notify(Event::Update, new_expense.clone());
        if let Some(expense) = self.expenses.iter_mut().find(|e| e.id == id) {
            *expense = new_expense;
        }
    }

    pub fn delete_expense(&mut self, id: u32) {
        self.expenses.retain(|expense| expense.id != id);
    }
}

impl Iterator for ExpenseManager {
    type Item = PartialExpense;

    // return only the expense id
    fn next(&mut self) -> Option<PartialExpense> {
        let expense = self.expenses.pop();

        if let Some(expense) = &expense {
            Some(PartialExpense {
                description: expense.description.clone(),
                amount: expense.amount,
                category: match &expense.category {
                    crate::Category::Food => "Food".to_string(),
                    crate::Category::Travel => "Travel".to_string(),
                    crate::Category::Utilities => "Utilities".to_string(),
                    crate::Category::Entertainment => "Entertainment".to_string(),
                    crate::Category::Other => "Other".to_string(),
                },
            })
        } else {
            None
        }
    }
}

impl fmt::Display for ExpenseManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Expense Manager")?;

        for expense in &self.expenses {
            write!(f, "{}", expense)?;
        }

        Ok(())
    }
}