use std::fmt::Display;

#[derive(Debug)]
pub struct Datas<T>
{
  pub name: T,
  pub sex: T,
  pub race: T,
  pub charactor: T,
  pub talent: T,
  pub camp: T,
  pub hobby: T,
  pub hair: T,
  pub pupil: T,
  pub danger: T,
  pub lucky: T
}

impl <T> Display for Datas<T>
where T: AsRef<str>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{
      {},
      "性别": {},
      "种族": {},
      "性格": {},
      "天赋能力": {},
      "爱好": {},
      "阵营": {},
      "发色": {},
      "瞳色": {},
      "危险度": {},
      "幸运": {}
    }}"#,
    self.name.as_ref(),
    self.sex.as_ref(),
    self.race.as_ref(),
    
    )
  }
}