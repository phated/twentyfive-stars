pub mod interfaces {
    use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, Wave};
    use async_graphql::{Interface, ID};

    #[derive(Interface)]
    #[graphql(field(name = "id", type = "ID"))]
    pub enum Node {
        BattleCard(BattleCard),
        CharacterCard(CharacterCard),
        StratagemCard(StratagemCard),
        // TODO: get methods supported on interfaces
        // CharacterMode,
        Wave(Wave),
    }

    #[derive(Clone, Debug, Interface)]
    #[graphql(
        field(name = "id", type = "ID"),
        field(name = "tcg_id", type = "&str"),
        field(name = "rarity", type = "CardRarity"),
        field(name = "number", type = "&str"),
        field(name = "category", type = "CardCategory"),
        field(name = "wave", type = "Wave")
    )]
    pub enum Card {
        BattleCard(BattleCard),
        CharacterCard(CharacterCard),
        StratagemCard(StratagemCard),
    }
}
