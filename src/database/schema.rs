table! {
    cards (id) {
        id -> Varchar,
        wave_id -> Varchar,
        title -> Varchar,
    }
}

table! {
    waves (id) {
        id -> Varchar,
        name -> Varchar,
        released -> Date,
    }
}

joinable!(cards -> waves (wave_id));

allow_tables_to_appear_in_same_query!(
    cards,
    waves,
);
