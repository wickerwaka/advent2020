pub use anyhow::{anyhow, Context, Error};
use std::path::Path;

pub trait AdventParse: Sized {
    fn parse(s: &str) -> Result<Self, Error>;
}

impl<T> AdventParse for T
where
    T: std::str::FromStr,
    T::Err: std::error::Error,
{
    fn parse(s: &str) -> Result<Self, Error> {
        s.parse::<Self>()
            .map_err(|err| anyhow!("Could not parse '{}': {}", s, err))
    }
}

pub fn read_list<A: AsRef<Path>, T: AdventParse>(path: A) -> Result<Vec<T>, Error> {
    let input =
        std::fs::read_to_string(path.as_ref()).context(format!("Reading: {:?}", path.as_ref()))?;

    let result: Result<Vec<T>, _> = input.lines().map(|x| T::parse(x)).collect();
    result
}
