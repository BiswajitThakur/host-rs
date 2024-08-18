use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::{error::Error, path::Path};

#[allow(unused)]
pub fn write_list<T, I, U>(path: T, value: I) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path>,
    I: IntoIterator<Item = U>,
    U: Display,
{
    let mut f = File::create(path)?;
    for line in value {
        writeln!(&mut f, "{}", line)?;
    }
    Ok(())
}
