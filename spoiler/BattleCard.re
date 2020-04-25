module BattleCardFragment = [%relay.fragment
  {|
  fragment BattleCard_battleCard on BattleCard {
    attackModifier
    defenseModifier
    faction
    icons
    stars
    title
    type_: type
  }
|}
];

[@react.component]
let make = (~card as cardRef) => {
  let card = BattleCardFragment.use(cardRef);
  <div> {React.string(card.title)} </div>;
};
