<!doctype html>
<html lang="en-US">
<head>
    <meta charset="utf-8"/>
    <title>Mortgage Calculator</title>
</head>
<body>
<form id="mortgage-form">
    <label for="principal">Principal:</label><br>
    <input type="number" id="principal" name="principal"><br>
    <label for="annual_interest_rate">Annual Interest Rate:</label><br>
    <input type="number" id="annual_interest_rate" name="annual_interest_rate"><br>
    <label for="years">Years:</label><br>
    <input type="number" id="years" name="years"><br>
    <label for="prepayment">Prepayment:</label><br>
    <input type="number" id="prepayment" name="prepayment"><br>
    <label for="num_payments">Number of Payments:</label><br>
    <input type="number" id="num_payments" name="num_payments"><br>
    <input type="submit" value="Calculate">
</form>
<p id="output"></p>
<script src="./index.js"></script>
</body>
</html>