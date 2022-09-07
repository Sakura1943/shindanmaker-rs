#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub sex: String,
    pub race: String,
    pub character: String,
    pub talent: String,
    pub camp: String,
    pub hobby: String,
    pub hair: String,
    pub pupil: String,
    pub danger: String,
    pub lucky: String,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            sex,
            race,
            character,
            talent,
            camp,
            hobby,
            hair,
            pupil,
            danger,
            lucky,
        } = self;
        write!(
            f,
            "{name},\
性别: {sex},\
种族: {race},\
性格: {character},\
天赋能力: {talent},\
阵营: {camp},\
爱好: {hobby},\
发色: {hair},\
瞳色: {pupil},\
危险度: {danger},\
幸运: {lucky}"
        )
    }
}
