use crate::Expense;
use serde_json;
use std::error::Error;

pub trait Exporter {
    fn export(&self, expenses: &[Expense]) -> Result<String, Box<dyn Error>>;
}

pub struct CsvExporter;

impl Exporter for CsvExporter {
    fn export(&self, expenses: &[Expense]) -> Result<String, Box<dyn Error>> {
        let mut csv_data = String::from("id,description,amount,category,date\n");

        for expense in expenses {
            csv_data.push_str(&format!(
                "{},{},{},{},{}\n",
                expense.id,
                expense.description,
                expense.amount,
                format!("{:?}", expense.category),
                expense
                    .date
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
            ));
        }

        Ok(csv_data)
    }
}

pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, expenses: &[Expense]) -> Result<String, Box<dyn Error>> {
        let json_data = serde_json::to_string(expenses)?;

        Ok(json_data)
    }
}
