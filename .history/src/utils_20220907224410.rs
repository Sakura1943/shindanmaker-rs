use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Datas<T>
{
  pub name: String,
  pub sex: String,
  pub race: String,
  pub charactor: String,
  pub talent: String,
  pub camp: String,
  pub hobby: String,
  pub hair: String,
  pub pupil: String,
  pub danger: String,
  pub lucky: String
}


impl <T> Display for Datas<T>
where T: ToString
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{
      {},
      性别: {},
      种族: {},
      性格: {},
      天赋能力: {},
      阵营: {},
      爱好: {},
      发色: {},
      瞳色: {},
      危险度: {},
      幸运: {}
}}"#,
    self.name.to_string(),
    self.sex.to_string(),
    self.race.to_string(),
    self.charactor.to_string(),
    self.talent.to_string(),
    self.camp.to_string(),
    self.hobby.to_string(),
    self.hair.to_string(),
    self.pupil.to_string(),
    self.danger.to_string(),
    self.lucky.to_string()
    )
  }
}