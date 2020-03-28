use crate::schema::Cursor;

pub enum Pagination {
  None,
  After(Cursor),
  Before(Cursor),
  Betwixt(Cursor, Cursor),
  First(i64),
  FirstAfter(i64, Cursor),
  FirstBefore(i64, Cursor),
  FirstBetwixt(i64, Cursor, Cursor),
  Last(i64),
  LastAfter(i64, Cursor),
  LastBefore(i64, Cursor),
  LastBetwixt(i64, Cursor, Cursor),
  Invalid,
}

struct Pag {
  first: Option<i32>,
  last: Option<i32>,
  before: Option<Cursor>,
  after: Option<Cursor>,
}

impl Pagination {
  pub fn new(
    first: Option<i32>,
    last: Option<i32>,
    before: Option<Cursor>,
    after: Option<Cursor>,
  ) -> Self {
    let pag = Pag {
      first,
      last,
      before,
      after,
    };

    match pag {
      // This is technically incorrect according to the GraphQL Spec, but highly discouraged
      Pag {
        first: Some(_first),
        last: Some(_last),
        before: _,
        after: _,
      } => Pagination::Invalid,
      Pag {
        first: None,
        last: None,
        before: None,
        after: None,
      } => Pagination::None,
      Pag {
        first: None,
        last: None,
        before: Some(before),
        after: None,
      } => Pagination::Before(before),
      Pag {
        first: None,
        last: None,
        before: None,
        after: Some(after),
      } => Pagination::After(after),
      Pag {
        first: None,
        last: None,
        before: Some(before),
        after: Some(after),
      } => Pagination::Betwixt(before, after),
      Pag {
        first: Some(first),
        last: None,
        before: None,
        after: None,
      } => Pagination::First(first.into()),
      Pag {
        first: Some(first),
        last: None,
        before: Some(before),
        after: None,
      } => Pagination::FirstBefore(first.into(), before),
      Pag {
        first: Some(first),
        last: None,
        before: None,
        after: Some(after),
      } => Pagination::FirstAfter(first.into(), after),
      Pag {
        first: Some(first),
        last: None,
        before: Some(before),
        after: Some(after),
      } => Pagination::FirstBetwixt(first.into(), before, after),
      Pag {
        first: None,
        last: Some(last),
        before: None,
        after: None,
      } => Pagination::Last(last.into()),
      Pag {
        first: None,
        last: Some(last),
        before: Some(before),
        after: None,
      } => Pagination::LastBefore(last.into(), before),
      Pag {
        first: None,
        last: Some(last),
        before: None,
        after: Some(after),
      } => Pagination::LastAfter(last.into(), after),
      Pag {
        first: None,
        last: Some(last),
        before: Some(before),
        after: Some(after),
      } => Pagination::LastBetwixt(last.into(), before, after),
    }
  }
}
