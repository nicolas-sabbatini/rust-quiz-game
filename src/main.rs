use anyhow::Result;
use clap::Parser;
use csv::ReaderBuilder;
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::File, io::stdin};

struct Question {
    question: String,
    answer: String,
}

impl Question {
    fn ask(&self) {
        println!("{}", self.question);
    }

    fn check(&self, answer: &str) -> bool {
        self.answer == answer
    }
}

struct Quiz {
    questions: Vec<Question>,
    correct: usize,
}

impl Quiz {
    fn ask(&mut self) {
        for (num, question) in self.questions.iter().enumerate() {
            println!("\nQuestion #{}:", num + 1);
            question.ask();
            let answer = read_line();
            if question.check(&answer) {
                self.correct += 1;
                println!("🎉 ¡Correct! 🎉");
            } else {
                println!("❌ Incorrect ❌");
            }
        }
    }

    fn result(&self) {
        println!("You got {} out of {}", self.correct, self.questions.len());
        if self.correct == self.questions.len() {
            println!("🎉🎉🎉 Congratulations you are a GENIUS 🎉🎉🎉");
        } else if self.correct == self.questions.len() / 2 {
            println!("👍 You are amazing 👍");
        } else {
            println!("👎 You need to study more 👎");
        }
    }

    fn from_cvs(file_path: &str) -> Result<Self> {
        let mut questions = vec![];
        let file = File::open(file_path)?;
        let mut cvs_reader = ReaderBuilder::new()
            .has_headers(false) // We don't have headers
            .flexible(true) // We want to allow different number of columns
            .from_reader(file);
        for row in cvs_reader.records() {
            let row = row.expect("Unable to read row");
            let mut row_iter = row.iter();
            let question = row_iter.next().expect("Unable to read question");
            let answer = row_iter.next().expect("Unable to read answer");
            questions.push(Question {
                question: question.to_string(),
                answer: answer.to_string(),
            });
        }
        questions.shuffle(&mut thread_rng());
        Ok(Quiz {
            questions,
            correct: 0,
        })
    }
}

fn read_line() -> String {
    let mut answer = String::new();
    loop {
        stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim().is_empty() {
            println!("Please enter an answer");
        } else {
            break;
        }
    }
    answer.trim().to_string()
}

/// A simple quiz game
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "input/test-1.csv")]
    cvs_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut quiz = Quiz::from_cvs(&args.cvs_path)?;
    quiz.ask();
    quiz.result();
    Ok(())
}
