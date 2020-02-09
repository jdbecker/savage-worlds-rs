use crate::character::attribute_type::AttributeType;
use crate::character::skill::Skill;

pub struct Skills {
    pub academics: Skill,
    pub athletics: Skill,
    pub battle: Skill,
    pub boating: Skill,
    pub common_knowledge: Skill,
    pub driving: Skill,
    pub electronics: Skill,
    pub faith: Skill,
    pub fighting: Skill,
    pub focus: Skill,
    pub gambling: Skill,
    pub hacking: Skill,
    pub healing: Skill,
    pub intimidation: Skill,
    pub language: Skill,
    pub notice: Skill,
    pub occult: Skill,
    pub performance: Skill,
    pub persuasion: Skill,
    pub piloting: Skill,
    pub psionics: Skill,
    pub repair: Skill,
    pub research: Skill,
    pub riding: Skill,
    pub science: Skill,
    pub shooting: Skill,
    pub spellcasting: Skill,
    pub stealth: Skill,
    pub survival: Skill,
    pub taunt: Skill,
    pub thievery: Skill,
    pub weird_science: Skill,
}

impl Default for Skills {
    fn default() -> Self {
        Skills {
            academics: Skill::untrained(AttributeType::Smarts),
            athletics: Skill::trained(AttributeType::Agility),
            battle: Skill::untrained(AttributeType::Smarts),
            boating: Skill::untrained(AttributeType::Agility),
            common_knowledge: Skill::trained(AttributeType::Smarts),
            driving: Skill::untrained(AttributeType::Agility),
            electronics: Skill::untrained(AttributeType::Smarts),
            faith: Skill::untrained(AttributeType::Spirit),
            fighting: Skill::untrained(AttributeType::Agility),
            focus: Skill::untrained(AttributeType::Spirit),
            gambling: Skill::untrained(AttributeType::Smarts),
            hacking: Skill::untrained(AttributeType::Smarts),
            healing: Skill::untrained(AttributeType::Smarts),
            intimidation: Skill::untrained(AttributeType::Spirit),
            language: Skill::untrained(AttributeType::Smarts),
            notice: Skill::trained(AttributeType::Smarts),
            occult: Skill::untrained(AttributeType::Smarts),
            performance: Skill::untrained(AttributeType::Spirit),
            persuasion: Skill::trained(AttributeType::Spirit),
            piloting: Skill::untrained(AttributeType::Agility),
            psionics: Skill::untrained(AttributeType::Smarts),
            repair: Skill::untrained(AttributeType::Smarts),
            research: Skill::untrained(AttributeType::Smarts),
            riding: Skill::untrained(AttributeType::Agility),
            science: Skill::untrained(AttributeType::Smarts),
            shooting: Skill::untrained(AttributeType::Agility),
            spellcasting: Skill::untrained(AttributeType::Smarts),
            stealth: Skill::trained(AttributeType::Agility),
            survival: Skill::untrained(AttributeType::Smarts),
            taunt: Skill::untrained(AttributeType::Smarts),
            thievery: Skill::untrained(AttributeType::Agility),
            weird_science: Skill::untrained(AttributeType::Smarts),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_core(skill: Skill) {
        assert_eq!(0, skill.modifier);
    }

    fn assert_untrained(skill: Skill) {
        assert_eq!(-2, skill.modifier);
    }

    #[test]
    fn check_core_skills() {
        let s = Skills::default();
        assert_core(s.athletics);
        assert_core(s.common_knowledge);
        assert_core(s.notice);
        assert_core(s.persuasion);
        assert_core(s.stealth);
    }

    #[test]
    fn check_untrained_skills() {
        let s = Skills::default();
        assert_untrained(s.academics);
        assert_untrained(s.battle);
        assert_untrained(s.boating);
        assert_untrained(s.driving);
        assert_untrained(s.electronics);
        assert_untrained(s.faith);
        assert_untrained(s.fighting);
        assert_untrained(s.focus);
        assert_untrained(s.gambling);
        assert_untrained(s.hacking);
        assert_untrained(s.healing);
        assert_untrained(s.intimidation);
        assert_untrained(s.language);
        assert_untrained(s.occult);
        assert_untrained(s.performance);
        assert_untrained(s.piloting);
        assert_untrained(s.psionics);
        assert_untrained(s.repair);
        assert_untrained(s.research);
        assert_untrained(s.riding);
        assert_untrained(s.science);
        assert_untrained(s.shooting);
        assert_untrained(s.spellcasting);
        assert_untrained(s.survival);
        assert_untrained(s.taunt);
        assert_untrained(s.thievery);
        assert_untrained(s.weird_science);
    }
}
