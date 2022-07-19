use hello::{english, russian, spanish};
use hello::french;

fn main() {
    english::greet();
    spanish::greet();
    russian::greet();
    french::greet();
}
