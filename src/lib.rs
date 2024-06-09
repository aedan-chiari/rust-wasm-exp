use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Mortgage {
    pub principal: f64,
    pub annual_interest_rate: f64,
    pub years: u32,
    pub prepayment: f64,
}

#[wasm_bindgen]
impl Mortgage {
    pub fn new(principal: f64, annual_interest_rate: f64, years: u32, prepayment: f64) -> Mortgage {
        Mortgage {
            principal,
            annual_interest_rate,
            years,
            prepayment,
        }
    }
    pub fn calculate_remaining_balance(&self, num_payments: u32) -> f64 {
        let monthly_interest_rate = self.annual_interest_rate / 12.0 / 100.0;
        let num_total_payments = (self.years * 12) as f64;

        let monthly_payment = self.principal
            * (monthly_interest_rate
                / (1.0 - (1.0 + monthly_interest_rate).powf(-num_total_payments)));

        let mut remaining_balance = self.principal;

        for _ in 0..num_payments {
            let interest = remaining_balance * monthly_interest_rate;
            let principal_payment = monthly_payment - interest;
            remaining_balance -= principal_payment;
            if remaining_balance < self.prepayment {
                remaining_balance = 0.0;
            } else {
                remaining_balance -= self.prepayment;
            }
           
        }
        remaining_balance
    }
}
