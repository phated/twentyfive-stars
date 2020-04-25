module AllCardsQuery = [%relay.query
  {|
  query AllCardsQuery {
    allCards {
      edges {
        node {
          __typename
          tcgId
          category
          number
          rarity
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
          // <div> {React.string(node.category)} </div>
          <div> {React.string(edge.node.number)} </div>
          {switch (edge.node.category) {
           | `BATTLE => <BattleCard card={edge.node.getFragmentRefs()} />
           | _ => React.null
           }}
        </div>
      })
    });

  React.array(children);
};
