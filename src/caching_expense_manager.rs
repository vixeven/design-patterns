use crate::{observer::Subscriber, Expense, ExpenseManager};

pub struct CachingExpenseManager {
    pub manager: ExpenseManager,
    pub subscriber: Subscriber,
    total_expenses_cache: Option<f64>,
}

impl CachingExpenseManager {
    pub fn new(manager: ExpenseManager) -> Self {
        CachingExpenseManager {
            manager,
            total_expenses_cache: None,
            subscriber: |expense| {
                println!("New expense modified: {:?}", expense);
            },
        }
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.manager.add_expense(expense);
        self.total_expenses_cache = None;
    }

    pub fn calculate_total_expenses(&mut self) -> f64 {
        if let Some(total) = self.total_expenses_cache {
            total
        } else {
            let total = self
                .manager
                .get_expenses()
                .iter()
                .map(|expense| expense.amount)
                .sum();

            self.total_expenses_cache = Some(total);

            return total;
        }
    }
}
