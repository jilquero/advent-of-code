use std::cmp::max;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Default, Debug, Clone)]
struct Game {
    #[allow(dead_code)]
    id: String,
    rounds: Vec<Cubes>,
}

impl Game {
    fn min_cubes(&self) -> Cubes {
        // alternatively
        // let max_colors = self
        //     .rounds
        //     .iter()
        //     .map(|cube| (cube.red, cube.green, cube.blue))
        //     .fold((0, 0, 0), |acc, (r, g, b)| {
        //         (max(acc.0, r), max(acc.1, g), max(acc.2, b))
        //     });

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        self.rounds.iter().for_each(|cube| {
            red = max(cube.red, red);
            green = max(cube.green, green);
            blue = max(cube.blue, blue);
        });

        Cubes { red, green, blue }
    }
}

#[derive(Default, Debug, Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

// 3 blue, 4 red
fn cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, cubes) =
        separated_list1(tag(", "), separated_pair(complete::u32, tag(" "), alpha1))(input)?;

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for (amount, color) in cubes {
        match color {
            "red" => red = amount,
            "green" => green = amount,
            "blue" => blue = amount,
            _ => panic!("unexpected value"),
        }
    }

    Ok((input, Cubes { red, green, blue }))
}

// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn rounds(input: &str) -> IResult<&str, Vec<Cubes>> {
    Ok(separated_list1(tag("; "), cubes)(input)?)
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), rounds)(input)?;
    let game = Game {
        id: id.to_owned(),
        rounds,
    };
    Ok((input, game))
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    Ok(separated_list1(newline, game)(input)?)
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games = games(input).expect("should parse").1;
    let output = games
        .iter()
        .map(|game| game.min_cubes())
        .map(|cube| cube.power())
        .sum::<u32>();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
