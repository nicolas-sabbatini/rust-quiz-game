use anyhow::Result;
use clap::Parser;
use crossbeam::channel::{bounded, Sender};
use csv::ReaderBuilder;
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::File, io::stdin, sync::Arc, thread, time::Duration};

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
    finished: bool,
    time: u64,
}

impl Quiz {
    fn ask(&mut self, sender: &Sender<()>) {
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
        self.finished = true;
        sender.send(()).expect("Unable to send message");
    }

    fn result(&self) {
        if !self.finished {
            println!("⏲️⏲️ Time is up! ⏲️⏲️");
            println!("Lets see how you did...");
        }
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
        let mut time = 30;
        for row in cvs_reader.records() {
            let row = row.expect("Unable to read row");
            if row.len() == 1 {
                time = row[0].parse().expect("Unable to parse time");
                continue;
            }
            let question = &row[0];
            let answer = &row[1];
            questions.push(Question {
                question: question.to_string(),
                answer: answer.to_string(),
            });
        }
        questions.shuffle(&mut thread_rng());
        Ok(Quiz {
            questions,
            correct: 0,
            finished: false,
            time,
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

    let quiz = Quiz::from_cvs(&args.cvs_path)?;
    let time = quiz.time;

    let quiz_arc = Arc::new(quiz);
    let quiz_arc_thread = quiz_arc.clone();

    let (tx, rx) = bounded(1);
    let tx2 = tx.clone();

    thread::spawn(move || unsafe {
        // Unsafe magic to get a mutable reference to the arc
        let quiz = &mut *std::ptr::addr_of!((*quiz_arc_thread)).cast_mut();
        quiz.ask(&tx);
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(time));
        tx2.send(()).expect("Unable to send message");
    });

    rx.recv().expect("Unable to receive message");
    quiz_arc.result();
    Ok(())
}
