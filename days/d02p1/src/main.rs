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

    pub fn is_possible(&self, init: &GamePull) -> bool {
        self.red <= init.red && self.green <= init.green && self.blue <= init.blue
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let initial = GamePull {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games_possibilities: Vec<_> = input
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
            let cols: bool = el
                .into_iter()
                .map(|col| {
                    GamePull::new(col.split(',').map(|col| col.trim()).collect())
                        .is_possible(&initial)
                })
                .all(|el| el);
            cols
        })
        .collect();

    let result = games_possibilities
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, el)| {
            if el {
                return acc + idx + 1;
            };
            acc
        });
    dbg!(result);
}
