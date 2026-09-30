/*
  Problem 5: Celsius to Fahrenheit

  Write a function that converts a temperature from Celsius (f64) to Fahrenheit using the formula F = C * 9/5 + 32.
  Return the result as f64.

  Run the tests for this problem with:
    cargo test --test celsius_fahrenheit_test
*/

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let f: f64 = c * 9.00 / 5.00 + 32.00;
    f
}
