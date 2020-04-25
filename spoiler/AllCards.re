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

[@react.component]
let make = () => {
  let queryData = AllCardsQuery.use(~variables=(), ());
  let edges = Belt.Option.getWithDefault(queryData.allCards.edges, [||]);
  let children =
    Belt.Array.keepMap(edges, edge => {
      Belt.Option.map(edge, edge => {
        <div key={edge.node.tcgId}>
          <div> {React.string(edge.node.tcgId)} </div>
          <div> <CardCategory card={edge.node.getFragmentRefs()} /> </div>
          <div> {React.string(edge.node.number)} </div>
          <Wave wave={edge.node.wave.getFragmentRefs()} />
          {switch (edge.node.category) {
           | `BATTLE => <BattleCard card={edge.node.getFragmentRefs()} />
           | _ => React.null
           }}
        </div>
      })
    });

  React.array(children);
};
