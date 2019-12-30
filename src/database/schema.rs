table! {
    cards (id) {
        id -> Uuid,
        card_number -> Varchar,
        // card_type -> Card_type,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        // rarity -> Card_rarity,
        wave_id -> Varchar,
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

allow_tables_to_appear_in_same_query!(cards, waves,);
