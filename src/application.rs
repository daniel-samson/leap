#[macro_export] macro_rules! cli {
    () => {
        fn main() {
            println!("The CLI is coming soon");
        }
    };
}

#[macro_export] macro_rules! web {
    () => {
        fn main() {
            println!("The web is coming soon");
        }
    };
}
