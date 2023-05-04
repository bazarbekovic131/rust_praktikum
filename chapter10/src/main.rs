use chapter10::{Summary, Tweet};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = maximum(number_list);
    println!("The largest number in the dataset is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = maximum(number_list);
    println!("The largest number in the dataset is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = maximum(&char_list);
    println!("The largest char is {}", result);

    let int = Point {x: 2, y: 3};
    let float = Point {x: 1.0, y: 3.3};
    let different = Point2 { x:4, y: 5.2};

    println!("int.x = {}", int.x());

    // chapter 10.2 traits

    let tweet = Tweet {
        username: String::from("SpringerDE"),
        content: String::from("Große Aktion für die begrenzte Zeit!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn maximum<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}