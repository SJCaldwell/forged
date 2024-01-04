
enum TaskDifficulty{
    Easy = 10,
    Medium = 15,
    Hard = 20,
    VeryHard = 25,
    NearlyImpossible = 30,
}

enum Skill {
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

pub struct SkillCheck{
    success: bool,
    task_difficulty: TaskDifficulty,
    relevant_skill: Skill,
}