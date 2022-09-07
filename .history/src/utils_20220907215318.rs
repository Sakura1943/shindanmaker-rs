use std::fmt::Display;

#[derive(Debug)]
pub struct Datas<T>
where T: AsRef<str>
{
  pub name: T,
  pub sex: T,
  pub race: T,
  pub charactor: T,
  pub talent: T,
  pub camp: T,
  pub hair: T,
  pub pupil: T,
  pub danger: T,
  pub lucky: T
}

impl <T: As>Display for Datas<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}