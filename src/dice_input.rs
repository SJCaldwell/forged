use regex::Regex;
use std::error::Error;
use rand::Rng;

pub struct DiceInput{
    num_dice: i32,
    num_sides: i32,
    modifier: i32,
}

impl DiceInput{
    pub fn parse(input: &str) -> Result<DiceInput, Box<dyn Error>>{
        let re = Regex::new(r"(\d+)d(\d+)(?: \+ (\d+))?").unwrap(); 
        match re.captures(input) {
            Some(caps) => {
                let num_dice: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let num_sides: i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let modifier: i32 = match caps.get(3) {
                    Some(m) => m.as_str().parse::<i32>().unwrap(),
                    None => 0
                };
                Ok(DiceInput::new(num_dice, num_sides, modifier))
            },
            None => {
                Err("Invalid input format".into())
            }
        }
    }

    pub fn new(num_dice: i32, num_sides: i32, modifier: i32) -> DiceInput{
        DiceInput{
            num_dice,
            num_sides,
            modifier,
        }
    }

    pub fn roll(&self) -> i32 {
        let mut total = 0;
        for _ in 0..self.num_dice {
            total += rand::thread_rng().gen_range(1..self.num_sides + 1);
        }
        total + self.modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input_with_modifier(){
        let result = DiceInput::parse("2d6 + 3").unwrap();
        assert_eq!(result.num_dice, 2);
        assert_eq!(result.num_sides, 6);
        assert_eq!(result.modifier, 3);
    }

    #[test]
    fn test_valid_input_without_modifier(){
        let result = DiceInput::parse("2d6").unwrap();
        assert_eq!(result.num_dice, 2);
        assert_eq!(result.num_sides, 6);
        assert_eq!(result.modifier, 0);
    }

    #[test]
    fn test_invalid_input() {
        let result = DiceInput::parse("1143");
        assert!(result.is_err());
    }
}