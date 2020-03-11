use crate::schema::Cursor;
use uuid::Uuid;

pub enum Pagination {
  None,
  After(Uuid),
  Before(Uuid),
  Betwixt(Uuid, Uuid),
  First(i32),
  FirstAfter(i32, Uuid),
  FirstBefore(i32, Uuid),
  FirstBetwixt(i32, Uuid, Uuid),
  Last(i32),
  LastAfter(i32, Uuid),
  LastBefore(i32, Uuid),
  LastBetwixt(i32, Uuid, Uuid),
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
        before: Some(Cursor(before)),
        after: None,
      } => Pagination::Before(before),
      Pag {
        first: None,
        last: None,
        before: None,
        after: Some(Cursor(after)),
      } => Pagination::After(after),
      Pag {
        first: None,
        last: None,
        before: Some(Cursor(before)),
        after: Some(Cursor(after)),
      } => Pagination::Betwixt(before, after),
      Pag {
        first: Some(first),
        last: None,
        before: None,
        after: None,
      } => Pagination::First(first),
      Pag {
        first: Some(first),
        last: None,
        before: Some(Cursor(before)),
        after: None,
      } => Pagination::FirstBefore(first, before),
      Pag {
        first: Some(first),
        last: None,
        before: None,
        after: Some(Cursor(after)),
      } => Pagination::FirstAfter(first, after),
      Pag {
        first: Some(first),
        last: None,
        before: Some(Cursor(before)),
        after: Some(Cursor(after)),
      } => Pagination::FirstBetwixt(first, before, after),
      Pag {
        first: None,
        last: Some(last),
        before: None,
        after: None,
      } => Pagination::Last(last),
      Pag {
        first: None,
        last: Some(last),
        before: Some(Cursor(before)),
        after: None,
      } => Pagination::LastBefore(last, before),
      Pag {
        first: None,
        last: Some(last),
        before: None,
        after: Some(Cursor(after)),
      } => Pagination::LastAfter(last, after),
      Pag {
        first: None,
        last: Some(last),
        before: Some(Cursor(before)),
        after: Some(Cursor(after)),
      } => Pagination::LastBetwixt(last, before, after),
    }
  }
}
