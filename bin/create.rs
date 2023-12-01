use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Error, Write},
    process,
};

const YEAR: i32 = 2023;

struct Puzzle {
    day: i32,
    year: i32,
}

impl Puzzle {
    fn create_boilerplate(&self) -> Result<(), Error> {
        let path = format!("src/aoc{}/day{}", self.year, self.day);
        let path = path.as_str();

        self.create_folder(path)?;
        self.create_data(path)?;
        self.create_code(path)?;
        self.add_defs()?;
        self.inject_main()?;

        Ok(())
    }

    fn create_folder(&self, path: &str) -> Result<(), Error> {
        println!("Creating folder: {path}");

        fs::create_dir(path)?;
        Ok(())
    }

    fn create_data(&self, path: &str) -> Result<(), Error> {
        let path = format!("{path}/data.txt");

        println!("Creating {path}");

        File::create(path)?;
        Ok(())
    }

    fn create_code(&self, path: &str) -> Result<(), Error> {
        let template = include_str!("template.txt");
        let path = format!("{path}/mod.rs");

        println!("Writing {path}");

        let mut code = File::create(path)?;
        code.write_all(template.as_bytes())?;

        Ok(())
    }

    fn add_defs(&self) -> Result<(), Error> {
        let path = format!("src/aoc{}/mod.rs", self.year);

        println!("Adding 'use' in {path}");

        let mut code = OpenOptions::new().append(true).open(path)?;
        let mod_decl = format!("pub mod day{};\n", self.day);
        code.write_all(mod_decl.as_bytes())?;

        Ok(())
    }

    fn inject_main(&self) -> Result<(), Error> {
        println!("Rewriting src/main.rs");

        let template = format!(include_str!("main_fmt.txt"), self.year, self.day);

        let mut code = OpenOptions::new().write(true).open("src/main.rs")?;
        code.write_all(template.as_bytes())?;

        Ok(())
    }
}

fn main() {
    let puzzle = parse_args().unwrap_or_else(|e| {
        eprintln!("ERROR! Unable to parse args: {e}");
        process::exit(1);
    });

    println!("... Creating day{} template", puzzle.day);

    puzzle.create_boilerplate().unwrap_or_else(|e| {
        eprintln!("ERROR! {e}");
        process::exit(1);
    });
}

fn parse_args() -> Result<Puzzle, String> {
    let mut args = env::args().skip(1);

    let year = args.next().map_or(Ok(YEAR), |year| {
        year.parse().map_err(|_| "Couldn't parse year")
    })?;

    let path = format!("./src/aoc{}/", year);
    let day: i32 = args.next().map_or_else(
        || {
            Ok::<i32, String>(
                fs::read_dir(path)
                    .map_err(|e| format!("Unable to read 'aoc{year}/': {e}"))?
                    .filter_map(|path| {
                        let file = path.unwrap().file_name();
                        let path = file.to_str().unwrap();
                        if path.contains("day") {
                            let number: i32 = path
                                .chars()
                                .filter(|&c| '0' <= c && c <= '9')
                                .collect::<String>()
                                .parse()
                                .unwrap();

                            return Some(number);
                        } else {
                            return None;
                        }
                    })
                    .max()
                    .unwrap_or(0)
                    + 1,
            )
        },
        |day| day.parse().map_err(|_| "Couldn't parse day".to_string()),
    )?;

    return Ok(Puzzle { day, year });
}
