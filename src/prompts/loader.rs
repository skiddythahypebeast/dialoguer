use std::future::Future;

use console::{style, Key, Term};
use spinoff::{spinners, Spinner};
use tokio::select;

pub struct Loader {
    prompt: String
}

impl Loader {
    pub fn new() -> Self {
        Self {
            prompt: "Loading".to_string()
        }
    }
    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();

        self
    }
    pub async fn interact_with_cancel<T>(&self, fut: impl Future<Output = T>) -> Option<T> {
        let term = Term::stderr();
        let mut spinner = Spinner::new(
            spinners::Moon, 
            format!("{} {}", self.prompt, style("| hit enter to cancel").dim()), 
            None
        );

        let exit_handle = tokio::spawn(async move {
            loop {
                match term.read_key() {
                    Ok(Key::Enter) => break,
                    _ => (),
                }
            };
        });

        select! {
            _ = exit_handle => {
                spinner.clear();

                return None;
            },
            fut = fut => {
                spinner.clear();

                return Some(fut)
            }
        };
    }
    pub async fn interact<T>(&self, fut: impl Future<Output = T>) -> T {
        let mut spinner = Spinner::new(
            spinners::Moon, 
            format!("{}", self.prompt), 
            None
        );

        let result = fut.await;

        spinner.clear();

        result
    }
}