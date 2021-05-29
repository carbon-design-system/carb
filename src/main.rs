use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
