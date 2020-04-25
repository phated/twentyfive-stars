module CardCategory = [%relay.fragment
  {|
  fragment CardCategory_card on Card {
    category
  }
|}
];

let toString = category =>
  switch (category) {
  | `BATTLE => "Battle"
  | `CHARACTER => "Character"
  | `STRATAGEM => "Stratagem"
  | `FutureAddedValue(a) => "Unknown value: " ++ a
  };

[@react.component]
let make = (~card as cardRef) => {
  let card = CardCategory.use(cardRef);
  React.string(toString(card.category));
};
