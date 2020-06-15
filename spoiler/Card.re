module CardFragement = [%relay.fragment
  {|
  fragment Card_card on Card {
    tcgId
    ...CardCategory_card
    wave {
      ...Wave_wave
    }
  }
|}
];

[@react.component]
let make = (~card) => {
  let card = CardFragement.use(card);

  <>
    <div> {React.string(card.tcgId)} </div>
    <div> <CardCategory card={card.getFragmentRefs()} /> </div>
    <Wave wave={card.wave.getFragmentRefs()} />
  </>;
};
