module BattleCardFragment = [%relay.fragment
  {|
  fragment BattleCard_battleCard on BattleCard {
    ...Card_card
  
    attackModifier
    defenseModifier
    faction
    icons
    stars
    title
    type_: type
    image {
      originalUrl
    }
  }
|}
];

module Styles = {
  open Emotion;

  let card = [%css []];

  let image = [%css [minWidth(200->px), maxWidth(100.0->pct)]];

  let info = [%css [minWidth(200->px)]];
};

[@react.component]
let make = (~card as cardRef) => {
  let card = BattleCardFragment.use(cardRef);

  let isActive = true;

  <div className=Styles.card>
    <div>
      {Belt.Option.mapWithDefault(card.image, React.null, image =>
         <img className=Styles.image src={image.originalUrl} />
       )}
    </div>
  </div>;
  // {isActive
  //    ? <div className=Styles.info>
  //        <div> {React.string(card.title)} </div>
  //        <br />
  //        <Card card={card.getFragmentRefs()} />
  //      </div>
  //    : React.null}
};
