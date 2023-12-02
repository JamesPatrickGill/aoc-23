#[derive(Debug)]
struct GamePull {
    red: u32,
    green: u32,
    blue: u32,
}

impl GamePull {
    pub fn new(input: Vec<&str>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for el in input {
            if el.contains("red") {
                red = el.replace("red", "").trim().parse::<u32>().unwrap()
            }
            if el.contains("blue") {
                blue = el.replace("blue", "").trim().parse::<u32>().unwrap()
            }
            if el.contains("green") {
                green = el.replace("green", "").trim().parse::<u32>().unwrap()
            }
        }

        GamePull { red, green, blue }
    }

    pub fn new_min(input: Vec<GamePull>) -> Self {
        let red = input.iter().max_by(|a, b| a.red.cmp(&b.red)).unwrap().red;
        let green = input
            .iter()
            .max_by(|a, b| a.green.cmp(&b.green))
            .unwrap()
            .green;
        let blue = input
            .iter()
            .max_by(|a, b| a.blue.cmp(&b.blue))
            .unwrap()
            .blue;
        GamePull { red, green, blue }
    }

    pub fn get_power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let res: u32 = input
        .lines()
        .map(|line| {
            let tmp: Vec<_> = line.split(':').collect();
            tmp[1].trim()
        })
        .map(|res| {
            let pull_str: Vec<_> = res.split(';').map(|el| el.trim()).collect();
            pull_str
        })
        .map(|el| {
            let cols: Vec<GamePull> = el
                .into_iter()
                .map(|col| GamePull::new(col.split(',').map(|col| col.trim()).collect()))
                .collect();
            cols
        })
        .map(|game| GamePull::new_min(game).get_power())
        .sum();

    dbg!(res);
}
