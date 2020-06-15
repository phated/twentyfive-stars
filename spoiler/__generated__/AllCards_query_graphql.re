/* @generated */

module Types = {
  type fragment_allCards_edges_node_BattleCard = {
    id: string,
    getFragmentRefs:
      unit =>
      {
        .
        "__$fragment_ref__BattleCard_battleCard": BattleCard_battleCard_graphql.t,
      },
  };
  type fragment_allCards_edges_node_CharacterCard = {id: string};
  type fragment_allCards_edges_node = [
    | `BattleCard(fragment_allCards_edges_node_BattleCard)
    | `CharacterCard(fragment_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ];
  type fragment_allCards_edges = {
    node: [
      | `BattleCard(fragment_allCards_edges_node_BattleCard)
      | `CharacterCard(fragment_allCards_edges_node_CharacterCard)
      | `UnselectedUnionMember(string)
    ],
  };
  type fragment_allCards = {
    edges: option(array(option(fragment_allCards_edges))),
  };

  type fragment = {allCards: fragment_allCards};
};

let unwrap_fragment_allCards_edges_node:
  {. "__typename": string} =>
  [
    | `BattleCard(Types.fragment_allCards_edges_node_BattleCard)
    | `CharacterCard(Types.fragment_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ] =
  u =>
    switch (u##__typename) {
    | "BattleCard" => `BattleCard(u->Obj.magic)
    | "CharacterCard" => `CharacterCard(u->Obj.magic)
    | v => `UnselectedUnionMember(v)
    };

let wrap_fragment_allCards_edges_node:
  [
    | `BattleCard(Types.fragment_allCards_edges_node_BattleCard)
    | `CharacterCard(Types.fragment_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ] =>
  {. "__typename": string} =
  fun
  | `BattleCard(v) => v->Obj.magic
  | `CharacterCard(v) => v->Obj.magic
  | `UnselectedUnionMember(v) => {"__typename": v};

module Internal = {
  type fragmentRaw;
  let fragmentConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"allCards_edges":{"n":"","na":""},"allCards_edges_node":{"u":"fragment_allCards_edges_node"},"allCards_edges_node_battlecard":{"f":""}}} |json}
  ];
  let fragmentConverterMap = {
    "fragment_allCards_edges_node": unwrap_fragment_allCards_edges_node,
  };
  let convertFragment = v =>
    v
    ->ReasonRelay._convertObj(
        fragmentConverter,
        fragmentConverterMap,
        Js.undefined,
      );
};

type t;
type fragmentRef;
type fragmentRefSelector('a) =
  {.. "__$fragment_ref__AllCards_query": t} as 'a;
external getFragmentRef: fragmentRefSelector('a) => fragmentRef = "%identity";

module Utils = {
  open Types;
  let getConnectionNodes_allCards:
    fragment_allCards => array(fragment_allCards_edges_node) =
    connection =>
      switch (connection.edges) {
      | None => [||]
      | Some(edges) =>
        edges
        ->Belt.Array.keepMap(edge =>
            switch (edge) {
            | None => None
            | Some(edge) => Some(edge.node)
            }
          )
      };
};

type operationType = ReasonRelay.fragmentNode;

let node: operationType = [%raw
  {json| (function(){
var v0 = [
  "allCards"
],
v1 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "id",
  "args": null,
  "storageKey": null
};
return {
  "kind": "Fragment",
  "name": "AllCards_query",
  "type": "QueryRoot",
  "metadata": {
    "connection": [
      {
        "count": "count",
        "cursor": "cursor",
        "direction": "forward",
        "path": (v0/*: any*/)
      }
    ],
    "refetch": {
      "connection": {
        "forward": {
          "count": "count",
          "cursor": "cursor"
        },
        "backward": null,
        "path": (v0/*: any*/)
      },
      "operation": require('./AllCardsRefetchQuery_graphql.bs.js').node,
      "fragmentPathInResult": []
    }
  },
  "argumentDefinitions": [
    {
      "kind": "LocalArgument",
      "name": "count",
      "type": "Int",
      "defaultValue": 20
    },
    {
      "kind": "LocalArgument",
      "name": "cursor",
      "type": "String",
      "defaultValue": ""
    }
  ],
  "selections": [
    {
      "kind": "LinkedField",
      "alias": "allCards",
      "name": "__AllCards_allCards_connection",
      "storageKey": null,
      "args": null,
      "concreteType": "CardsConnection",
      "plural": false,
      "selections": [
        {
          "kind": "LinkedField",
          "alias": null,
          "name": "edges",
          "storageKey": null,
          "args": null,
          "concreteType": "CardsEdge",
          "plural": true,
          "selections": [
            {
              "kind": "LinkedField",
              "alias": null,
              "name": "node",
              "storageKey": null,
              "args": null,
              "concreteType": null,
              "plural": false,
              "selections": [
                {
                  "kind": "ScalarField",
                  "alias": null,
                  "name": "__typename",
                  "args": null,
                  "storageKey": null
                },
                {
                  "kind": "InlineFragment",
                  "type": "BattleCard",
                  "selections": [
                    (v1/*: any*/),
                    {
                      "kind": "FragmentSpread",
                      "name": "BattleCard_battleCard",
                      "args": null
                    }
                  ]
                },
                {
                  "kind": "InlineFragment",
                  "type": "CharacterCard",
                  "selections": [
                    (v1/*: any*/)
                  ]
                }
              ]
            },
            {
              "kind": "ScalarField",
              "alias": null,
              "name": "cursor",
              "args": null,
              "storageKey": null
            }
          ]
        },
        {
          "kind": "LinkedField",
          "alias": null,
          "name": "pageInfo",
          "storageKey": null,
          "args": null,
          "concreteType": "PageInfo",
          "plural": false,
          "selections": [
            {
              "kind": "ScalarField",
              "alias": null,
              "name": "endCursor",
              "args": null,
              "storageKey": null
            },
            {
              "kind": "ScalarField",
              "alias": null,
              "name": "hasNextPage",
              "args": null,
              "storageKey": null
            }
          ]
        }
      ]
    }
  ]
};
})() |json}
];
