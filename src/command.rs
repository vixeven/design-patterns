use crate::{Expense, ExpenseManager};

pub trait Command {
    fn execute(&self, manager: &mut ExpenseManager);
}

/**
 * Command for adding an expense
 */
pub struct AddExpenseCommand {
    expense: Expense,
}

impl AddExpenseCommand {
    pub fn new(expense: Expense) -> Self {
        AddExpenseCommand { expense }
    }
}

impl Command for AddExpenseCommand {
    fn execute(&self, manager: &mut ExpenseManager) {
        manager.add_expense(self.expense.clone());
    }
}

/**
 * Command for editing an expense
 */
pub struct EditExpenseCommand {
    id: u32,
    new_expense: Expense,
}

impl EditExpenseCommand {
    pub fn new(id: u32, new_expense: Expense) -> Self {
        EditExpenseCommand { id, new_expense }
    }
}

impl Command for EditExpenseCommand {
    fn execute(&self, manager: &mut ExpenseManager) {
        manager.edit_expense(self.id, self.new_expense.clone());
    }
}

/**
 * Command for deleting an expense
 */
pub struct DeleteExpenseCommand {
    id: u32,
}

impl DeleteExpenseCommand {
    pub fn new(id: u32) -> Self {
        DeleteExpenseCommand { id }
    }
}

impl Command for DeleteExpenseCommand {
    fn execute(&self, manager: &mut ExpenseManager) {
        manager.delete_expense(self.id);
    }
}
