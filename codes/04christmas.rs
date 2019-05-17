fn main() {
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    // taking advantage of the repetition in the song
    let sequence = [
        "first", "second", "third", "fourth", "fifth", "sixth",
         "seventh", "eighth", "ninth", "tenth", "eleven", "twelfth"];
    let presents = [
        "a partridge in a pear tree", "two turtle doves", "three French hens",
        "four calling birds", "five gold rings", "six geese a laying",
        "seven swans a swimming", "eight maids a milking", "nine drummers drumming",
        "ten pipers piping", "eleven ladies dancing", "twelve Lords a leaping"];
    let mut text = String::new();

    for number in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", sequence[number]);
        if number == 0 {
            println!("{}", presents[number]);
            text = format!("and {}", presents[number]);
        }
        else {
            text = format!("{}, {}", presents[number], text);
            println!("{}", text);
        }
        println!();
    }
}
