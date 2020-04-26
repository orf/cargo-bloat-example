use chrono::Duration;
use chrono_humanize::{Accuracy, HumanTime, Tense};

fn main() {
    let dt = Duration::days(45);
    let ht = HumanTime::from(dt);
    println!("{}", ht.to_text_en(Accuracy::Precise, Tense::Present));
}
