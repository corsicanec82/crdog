use human_panic::setup_panic;

mod cli;

fn main() {
    setup_panic!();

    cli::run();
}
