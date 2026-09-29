/*
  Problem 2: Shadowing Transformer

  Write a function that takes a u32, doubles it, then converts the result to a String.
  You should shadow the variable at each step within the function body.

  Run the tests for this problem with:
    cargo test --test shadowing_test
*/

pub fn shadow_transform(x: u32) -> String {
    let y = x * 2;
    let s = y.to_string();
    s
}
