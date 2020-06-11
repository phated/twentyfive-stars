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

  let card = [%css
    [
      padding(rem(0.5)),
      boxShadow(~x=px(5), ~y=px(5), ~blur=px(5), rgba(0, 0, 0, 0.1)),
    ]
  ];

  let image = [%css [maxWidth(pct(100.0))]];
};

[@react.component]
let make = (~card as cardRef) => {
  let card = BattleCardFragment.use(cardRef);

  <div className=Styles.card>
    {Belt.Option.mapWithDefault(card.image, React.null, image =>
       <img className=Styles.image src={image.originalUrl} />
     )}
    <div> {React.string(card.title)} </div>
    <br />
    <Card card={card.getFragmentRefs()} />
  </div>;
};
