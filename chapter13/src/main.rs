use std::thread;

fn main() {
    let store = Inventar {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // let add_one_v4 = |x| {x + 1}; closure definition example

    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    let only_borrows = || {
        list.push(7);
        println!("From closure: {:?}", list)
    };

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventar {
    shirts: Vec<ShirtColour>,
}

impl Inventar {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColour {
        let mut n_red = 0;
        let mut n_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => n_red +=1,
                ShirtColour::Blue => n_blue +=1,
            }
        }

        if n_red > n_blue {
            ShirtColour::Red
        }
        else {
            ShirtColour::Blue
        }
    }
}