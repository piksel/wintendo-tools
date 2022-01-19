use std::error::{Error};

pub struct Woe {
    message: &'static str,
}

impl Woe {
    pub fn new(message: &'static str) -> Woe {
        Woe{message}
    }

    pub fn result<T>(message: &'static str) -> Result<T, Box<dyn Error>> {
        Err(Box::new(Woe{message}))
    }
}

impl std::fmt::Debug for Woe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
        // f.debug_struct("Woe").field("message", &self.message).finish()
    }
}

impl std::fmt::Display for Woe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message)
    }
}

impl Error for Woe {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        self.message
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
} 