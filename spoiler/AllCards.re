module AllCardsQuery = [%relay.query
  {|
  query AllCardsQuery {
    allCards {
      edges {
        node {
          __typename
  
          ... on BattleCard {
            id
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
        switch (edge.node) {
        | `BattleCard(card) =>
          <BattleCard key={card.id} card={card.getFragmentRefs()} />
        | _ => React.null
        }
      })
    });

  React.array(children);
};
