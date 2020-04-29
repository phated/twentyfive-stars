module AllCardsQuery = [%relay.query
  {|
  query AllCardsQuery {
    allCards {
      edges {
        node {
          __typename
          tcgId
          # It's weird that I need to request the `category` field AND the fragment
          category
          ...CardCategory_card
          number
          rarity
          wave {
            ...Wave_wave
          }
          ... on BattleCard {
            ...BattleCard_battleCard
          }
          ... on CharacterCard {
            id
          }
        }
      }
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
};

[@react.component]
let make = () => {
  let queryData = AllCardsQuery.use(~variables=(), ());
  let edges = Belt.Option.getWithDefault(queryData.allCards.edges, [||]);
  let children =
    Belt.Array.keepMap(edges, edge => {
      Belt.Option.map(edge, edge => {
        <div className=Styles.card key={edge.node.tcgId}>
          {switch (edge.node.category) {
           | `BATTLE => <BattleCard card={edge.node.getFragmentRefs()} />
           | _ => React.null
           }}
          <div> {React.string(edge.node.tcgId)} </div>
          <div> <CardCategory card={edge.node.getFragmentRefs()} /> </div>
          <Wave wave={edge.node.wave.getFragmentRefs()} />
        </div>
      })
    });

  React.array(children);
};
