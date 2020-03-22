mod config;
pub mod darwaza;
pub mod proxyficate;
mod urlmap;

fn main() -> Result<(), std::io::Error> {
    darwaza::demain();
    Ok(())
}
