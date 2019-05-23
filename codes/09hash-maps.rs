use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team = String::from("Yellow");
    // let score = scores.get(&team);

    // match score {
    //     Some(number) => println!("{} has scored {}", team, number),
    //     None => println!("{} not found", team)
    // }

    for (key, value) in &scores {
        println!("{} has scored {}", key, value);
    }

    scores.insert(String::from("Blue"), 20);

    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(30);
    {
        let update = scores.entry(String::from("Yellow")).or_insert(99);
        *update -= 12;
    }

    println!("{:?}", scores);

}
