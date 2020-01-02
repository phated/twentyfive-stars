table! {
    use diesel::sql_types::*;
    use crate::database::types::{CardRarity, CardCategory};

    cards (id) {
        id -> Uuid,
        tcg_id -> Varchar,
        rarity -> CardRarity,
        number -> Varchar,
        category -> CardCategory,
        wave_id -> Uuid,
    }
}

table! {
    waves (id) {
        id -> Uuid,
        tcg_id -> Varchar,
        name -> Varchar,
        released -> Date,
    }
}

joinable!(cards -> waves (wave_id));

allow_tables_to_appear_in_same_query!(cards, waves,);
