import * as wasm from "../pkg/wasm_rust.js";

document.getElementById('mortgage-form').addEventListener('submit', function (event) {
    event.preventDefault();
    const principal = parseFloat(document.getElementById('principal').value);
    const annual_interest_rate = parseFloat(document.getElementById('annual_interest_rate').value);
    const years = parseInt(document.getElementById('years').value);
    const prepayment = parseFloat(document.getElementById('prepayment').value);
    const num_payments = parseInt(document.getElementById('num_payments').value);

    // Log the inputs
    console.log(`Principal: ${principal}`);
    console.log(`Annual Interest Rate: ${annual_interest_rate}`);
    console.log(`Years: ${years}`);
    console.log(`Prepayment: ${prepayment}`);
    console.log(`Number of Payments: ${num_payments}`);

    const mortgage = wasm.Mortgage.new(principal, annual_interest_rate, years, prepayment);
    const remaining_balance = mortgage.calculate_remaining_balance(num_payments);

    document.getElementById('output').innerText = `Remaining balance: ${remaining_balance}`;
});