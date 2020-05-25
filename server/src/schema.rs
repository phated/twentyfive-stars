pub mod interfaces {
    use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, Wave};
    use async_graphql::ID;

    #[async_graphql::Interface(field(name = "id", type = "ID"))]
    pub enum Node {
        BattleCard(BattleCard),
        CharacterCard(CharacterCard),
        StratagemCard(StratagemCard),
        // TODO: get methods supported on interfaces
        // CharacterMode,
        Wave(Wave),
    }

    #[async_graphql::Interface(
        field(name = "id", type = "ID"),
        field(name = "tcg_id", type = "&str"),
        field(name = "rarity", type = "CardRarity"),
        field(name = "number", type = "&str"),
        field(name = "category", type = "CardCategory"),
        field(name = "wave", type = "Wave")
    )]
    #[derive(Clone, Debug)]
    pub enum Card {
        BattleCard(BattleCard),
        CharacterCard(CharacterCard),
        StratagemCard(StratagemCard),
    }
}
