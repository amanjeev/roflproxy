mod config;
mod darwaza;
mod proxyficate;

fn main() -> Result<(), std::io::Error> {
    darwaza::demain();
    Ok(())
}
