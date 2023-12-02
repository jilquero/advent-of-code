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
    id: String,
    rounds: Vec<Cubes>,
}

impl Game {
    fn is_valid(&self) -> bool {
        !self
            .rounds
            .iter()
            .any(|cubes| cubes.red > 12 || cubes.green > 13 || cubes.blue > 14)
    }
}

#[derive(Default, Debug, Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
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
        .filter(|game| game.is_valid())
        .filter_map(|game| game.id.parse::<u32>().ok())
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
