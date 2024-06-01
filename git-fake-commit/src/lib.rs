use anyhow::Result;
use git2::{Config, Signature};

pub struct Commit<'a> {
    pub message: String,
    pub author: Signature<'a>,
}

pub struct CommitBuilder {
    message: Option<String>,
    author: Option<String>,
    email: Option<String>,
}

impl<'a> CommitBuilder {
    pub fn new() -> Self {
        CommitBuilder {
            message: None,
            author: None,
            email: None,
        }
    }

    pub fn message(mut self, message: Option<String>) -> Self {
        self.message = message;
        self
    }

    pub fn author(mut self, author: Option<String>) -> Self {
        self.author = author;
        self
    }

    pub fn email(mut self, email: Option<String>) -> Self {
        self.email = email;
        self
    }

    pub fn build(self) -> Result<Commit<'a>> {
        let config = Config::open_default()?;

        let author = match self.author {
            Some(author) => author,
            None => config.get_string("user.name")?,
        };

        let email = match self.email {
            Some(email) => email,
            None => config.get_string("user.email")?,
        };

        let message = match self.message {
            Some(msg) => msg,
            None => String::from("(chore) git-fake-commit"),
        };

        Ok(Commit {
            message,
            author: Signature::now(&author, &email)?,
        })
    }
}
