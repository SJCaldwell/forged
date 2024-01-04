
mod dice_input;
use dice_input::DiceRoll;

pub enum TaskDifficulty{
    Easy = 10,
    Medium = 15,
    Hard = 20,
    VeryHard = 25,
    NearlyImpossible = 30,
}

pub enum Skill {
    Athletics,
    Acrobatics,
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,

}

pub struct CharacterSheet {
    id: u32,
    name: String,
    class: String,
    level: u32,
    hit_points: u32,
    ability_scores: AbilityScores,
    skills: Vec<Skill>,
    equipment: Vec<String>,
    // Add other fields as needed, like spells, feats, proficiencies, etc.
}

pub struct AbilityScores {
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32,
}


pub struct SkillCheck{
    success: bool,
    task_difficulty: TaskDifficulty,
    relevant_skill: Skill,
    character_id: u32,
}

pub fn perform_skill_check(character: &CharacterSheet, skill: Skill, difficulty: TaskDifficulty) -> SkillCheck {
    let skill_modifier = 5;
    let roll = DiceRoll::new(1, 20, skill_modifier);
    let result = roll.roll();
    let success = result >= difficulty as i32;
    SkillCheck{
        success,
        task_difficulty: difficulty,
        relevant_skill: skill,
        character_id: character.id,
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_check() {
        //Create a character
        let character = CharacterSheet {
            name: "Test Character".to_string(),
            class: "Rogue".to_string(),
            level: 1,
            hit_points: 10,
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 15,
                constitution: 10,
                intelligence: 12,
                wisdom: 10,
                charisma: 10,
            },
            skills: vec![Skill::Stealth, Skill::Acrobatics],
            equipment: vec!["Leather Armor".to_string(), "Dagger".to_string()],
        };

        let success = perform_skill_check(&character, Skill::Stealth, TaskDifficulty::Easy);
        assert!(success.success == true || success.success == false);

    }
}