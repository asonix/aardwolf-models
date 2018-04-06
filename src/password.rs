use std::fmt;

use bcrypt::{hash, verify, DEFAULT_COST};

pub trait Verify {
    fn verify(&self, &str) -> Result<(), VerificationError>;
}

pub trait Create: Sized {
    fn create(&str) -> Result<Self, CreationError>;
}

#[derive(Debug, Clone, Copy, Eq, Fail, PartialEq)]
pub enum VerificationError {
    #[fail(display = "Error validating password")]
    Process,
    #[fail(display = "Invalid password")]
    Password,
}

#[derive(Debug, Clone, Copy, Eq, Fail, PartialEq)]
#[fail(display = "Error creating password")]
pub struct CreationError;

pub struct Password(String);

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl Verify for Password {
    fn verify(&self, given_password: &str) -> Result<(), VerificationError> {
        verify(&self.0, given_password)
            .map_err(|e| {
                error!("Error verifying password: {}", e);

                VerificationError::Process
            })
            .and_then(|verified| {
                if verified {
                    Ok(())
                } else {
                    Err(VerificationError::Password)
                }
            })
    }
}

impl Create for Password {
    fn create(password: &str) -> Result<Password, CreationError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| {
                error!("Error creating password: {}", e);

                CreationError
            })
            .map(Password)
    }
}
