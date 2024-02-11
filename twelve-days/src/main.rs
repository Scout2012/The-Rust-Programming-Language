fn main() {
    let days_of_christmas = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let days = [
        "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];

    let mut received: Vec<String> = Vec::new();

    for day in 1..=12 {
        println!("Verse {day}\n");
        println!("on the {} day of Christmas", days[day - 1]);
        println!("my true love sent to me");

        let newest_gift = format!("{} {}", days[day - 1], days_of_christmas[day - 1]);
        received.push(newest_gift);

        for gift in &received {
            println!("{gift}");
        }
        println!("");
    }
}
