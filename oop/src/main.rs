use oop::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

use oop::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 120,
                height: 150,
                options: vec![
                    String::from("Ja"),
                    String::from("Nein"),
                ],
            }),
            Box::new(Button {
                width: 30,
                height: 60,
                label: String::from("Klicken"),
            }),
        ],
    };

    screen.run();
}
