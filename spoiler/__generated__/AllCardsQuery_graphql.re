/* @generated */

module Types = {
  type response_allCards_edges_node_BattleCard = {
    id: string,
    getFragmentRefs:
      unit =>
      {
        .
        "__$fragment_ref__BattleCard_battleCard": BattleCard_battleCard_graphql.t,
      },
  };
  type response_allCards_edges_node_CharacterCard = {id: string};
  type response_allCards_edges_node = [
    | `BattleCard(response_allCards_edges_node_BattleCard)
    | `CharacterCard(response_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ];
  type response_allCards_edges = {
    node: [
      | `BattleCard(response_allCards_edges_node_BattleCard)
      | `CharacterCard(response_allCards_edges_node_CharacterCard)
      | `UnselectedUnionMember(string)
    ],
  };
  type response_allCards = {
    edges: option(array(option(response_allCards_edges))),
  };

  type response = {allCards: response_allCards};
  type variables = unit;
};

let unwrap_response_allCards_edges_node:
  {. "__typename": string} =>
  [
    | `BattleCard(Types.response_allCards_edges_node_BattleCard)
    | `CharacterCard(Types.response_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ] =
  u =>
    switch (u##__typename) {
    | "BattleCard" => `BattleCard(u->Obj.magic)
    | "CharacterCard" => `CharacterCard(u->Obj.magic)
    | v => `UnselectedUnionMember(v)
    };

let wrap_response_allCards_edges_node:
  [
    | `BattleCard(Types.response_allCards_edges_node_BattleCard)
    | `CharacterCard(Types.response_allCards_edges_node_CharacterCard)
    | `UnselectedUnionMember(string)
  ] =>
  {. "__typename": string} =
  fun
  | `BattleCard(v) => v->Obj.magic
  | `CharacterCard(v) => v->Obj.magic
  | `UnselectedUnionMember(v) => {"__typename": v};

module Internal = {
  type responseRaw;
  let responseConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"allCards_edges":{"n":"","na":""},"allCards_edges_node":{"u":"response_allCards_edges_node"},"allCards_edges_node_battlecard":{"f":""}}} |json}
  ];
  let responseConverterMap = {
    "response_allCards_edges_node": unwrap_response_allCards_edges_node,
  };
  let convertResponse = v =>
    v
    ->ReasonRelay._convertObj(
        responseConverter,
        responseConverterMap,
        Js.undefined,
      );

  let variablesConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {} |json}
  ];
  let variablesConverterMap = ();
  let convertVariables = v =>
    v
    ->ReasonRelay._convertObj(
        variablesConverter,
        variablesConverterMap,
        Js.undefined,
      );
};

type preloadToken;

module Utils = {};

type operationType = ReasonRelay.queryNode;

let node: operationType = [%raw
  {json| (function(){
var v0 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "__typename",
  "args": null,
  "storageKey": null
},
v1 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "id",
  "args": null,
  "storageKey": null
},
v2 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "tcgId",
  "args": null,
  "storageKey": null
};
return {
  "kind": "Request",
  "fragment": {
    "kind": "Fragment",
    "name": "AllCardsQuery",
    "type": "QueryRoot",
    "metadata": null,
    "argumentDefinitions": [],
    "selections": [
      {
        "kind": "LinkedField",
        "alias": null,
        "name": "allCards",
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
                  (v0/*: any*/),
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
              }
            ]
          }
        ]
      }
    ]
  },
  "operation": {
    "kind": "Operation",
    "name": "AllCardsQuery",
    "argumentDefinitions": [],
    "selections": [
      {
        "kind": "LinkedField",
        "alias": null,
        "name": "allCards",
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
                  (v0/*: any*/),
                  (v1/*: any*/),
                  {
                    "kind": "InlineFragment",
                    "type": "BattleCard",
                    "selections": [
                      (v2/*: any*/),
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "category",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "LinkedField",
                        "alias": null,
                        "name": "wave",
                        "storageKey": null,
                        "args": null,
                        "concreteType": "Wave",
                        "plural": false,
                        "selections": [
                          {
                            "kind": "ScalarField",
                            "alias": null,
                            "name": "name",
                            "args": null,
                            "storageKey": null
                          },
                          (v2/*: any*/),
                          {
                            "kind": "ScalarField",
                            "alias": null,
                            "name": "released",
                            "args": null,
                            "storageKey": null
                          },
                          (v1/*: any*/)
                        ]
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "attackModifier",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "defenseModifier",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "faction",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "icons",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "stars",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "title",
                        "args": null,
                        "storageKey": null
                      },
                      {
                        "kind": "ScalarField",
                        "alias": "type_",
                        "name": "type",
                        "args": null,
                        "storageKey": null
                      }
                    ]
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  },
  "params": {
    "operationKind": "query",
    "name": "AllCardsQuery",
    "id": null,
    "text": "query AllCardsQuery {\n  allCards {\n    edges {\n      node {\n        __typename\n        ... on BattleCard {\n          id\n          ...BattleCard_battleCard\n        }\n        ... on CharacterCard {\n          id\n        }\n        ... on Node {\n          id\n        }\n      }\n    }\n  }\n}\n\nfragment BattleCard_battleCard on BattleCard {\n  ...Card_card\n  attackModifier\n  defenseModifier\n  faction\n  icons\n  stars\n  title\n  type_: type\n}\n\nfragment CardCategory_card on Card {\n  category\n}\n\nfragment Card_card on Card {\n  tcgId\n  ...CardCategory_card\n  wave {\n    ...Wave_wave\n    id\n  }\n}\n\nfragment Wave_wave on Wave {\n  name\n  tcgId\n  released\n}\n",
    "metadata": {}
  }
};
})() |json}
];
