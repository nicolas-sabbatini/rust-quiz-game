# Rust Quiz Game
## Description
This is the implementation of the frist exercise of the [Gophercises](https://gophercises.com/) course.  
It is a simple quiz game that reads a CSV file with questions and answers and asks the user to answer them.  
The CSV must contain the limit for the quiz in the first row (see [example quiz](input/test-1.csv)).
The quiz is over when the time limit is reached or when the user answers all the questions.  
At the end of the quiz the user can see how many questions he answered correctly.

## Usage
```bash
cargo run
```
or
```bash
cargo run -- -c input/test-2.csv
```
