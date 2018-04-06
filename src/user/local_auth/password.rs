use std::fmt;
use std::io::Write;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::backend::Backend;
use diesel::Expression;
use diesel::serialize;
use diesel::deserialize;
use diesel::sql_types::Text;

pub(crate) trait Verify {
    fn verify(&self, PlaintextPassword) -> Result<(), VerificationError>;
}

pub(crate) trait Create: Sized {
    fn create(PlaintextPassword) -> Result<Self, CreationError>;
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum VerificationError {
    #[fail(display = "Error validating password")]
    Process,
    #[fail(display = "Invalid password")]
    Password,
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
#[fail(display = "Error creating password")]
pub struct CreationError;

#[derive(Deserialize)]
pub struct PlaintextPassword(String);

impl fmt::Debug for PlaintextPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for PlaintextPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

#[derive(AsExpression)]
pub struct Password(String);

impl Expression for Password {
    type SqlType = Text;
}

impl<DB> serialize::ToSql<Text, DB> for Password
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&self.0, out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Password
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).map(Password)
    }
}

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
    fn verify(&self, given_password: PlaintextPassword) -> Result<(), VerificationError> {
        verify(&self.0, &given_password.0)
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
    fn create(password: PlaintextPassword) -> Result<Password, CreationError> {
        hash(&password.0, DEFAULT_COST)
            .map_err(|e| {
                error!("Error creating password: {}", e);

                CreationError
            })
            .map(Password)
    }
}
