extern crate trait_test;

use trait_test::*;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!(
            "The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.",
            self.high_temp, self.low_temp, self.chance_of_precipitation
        )
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}
