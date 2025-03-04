use std::{future::Future, sync::Arc};

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
    pub async fn interact_with_cancel<R>(&self, fut: impl Future<Output = R>) -> Option<R> {
        let term = Arc::new(Term::stderr());
        let (request_sender, request_receiver) = oneshot::channel::<()>();

        let mut spinner = Spinner::new(
            spinners::Moon, 
            format!("{} {}", self.prompt, style("| hit enter to cancel").dim()), 
            None
        );

        let exit_term = Arc::clone(&term);
        let exit_handle = tokio::spawn(async move {
            loop {
                if request_receiver.try_recv().is_ok() {
                    return;
                }
                match exit_term.read_key() {
                    Ok(Key::Enter) => return,
                    _ => (),
                }
            };
        });

        select! {
            _ = exit_handle => {
                spinner.clear();
                term.flush().unwrap();

                return None;
            },
            fut = fut => {
                spinner.clear();
                
                request_sender.send(()).unwrap();
                term.flush().unwrap();

                return Some(fut)
            }
        };
    }
    pub async fn interact<R>(&self, fut: impl Future<Output = R>) -> R {
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