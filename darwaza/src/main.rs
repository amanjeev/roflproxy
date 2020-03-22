mod config;
mod darwaza;
mod proxyficate;
mod urlmap;

fn main() -> Result<(), std::io::Error> {
    darwaza::demain();
    Ok(())
}
