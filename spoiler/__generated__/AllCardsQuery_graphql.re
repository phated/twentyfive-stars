/* @generated */

type enum_CardRarity = [
  | `COMMON
  | `PROMO
  | `RARE
  | `SUPER_RARE
  | `THEME
  | `UNCOMMON
  | `FutureAddedValue(string)
];

let unwrap_enum_CardRarity: string => enum_CardRarity =
  fun
  | "COMMON" => `COMMON
  | "PROMO" => `PROMO
  | "RARE" => `RARE
  | "SUPER_RARE" => `SUPER_RARE
  | "THEME" => `THEME
  | "UNCOMMON" => `UNCOMMON
  | v => `FutureAddedValue(v);

let wrap_enum_CardRarity: enum_CardRarity => string =
  fun
  | `COMMON => "COMMON"
  | `PROMO => "PROMO"
  | `RARE => "RARE"
  | `SUPER_RARE => "SUPER_RARE"
  | `THEME => "THEME"
  | `UNCOMMON => "UNCOMMON"
  | `FutureAddedValue(v) => v;

type enum_CardCategory = [
  | `BATTLE
  | `CHARACTER
  | `STRATAGEM
  | `FutureAddedValue(string)
];

let unwrap_enum_CardCategory: string => enum_CardCategory =
  fun
  | "BATTLE" => `BATTLE
  | "CHARACTER" => `CHARACTER
  | "STRATAGEM" => `STRATAGEM
  | v => `FutureAddedValue(v);

let wrap_enum_CardCategory: enum_CardCategory => string =
  fun
  | `BATTLE => "BATTLE"
  | `CHARACTER => "CHARACTER"
  | `STRATAGEM => "STRATAGEM"
  | `FutureAddedValue(v) => v;

module Types = {
  type response_allCards_edges_node_wave = {
    getFragmentRefs:
      unit => {. "__$fragment_ref__Wave_wave": Wave_wave_graphql.t},
  };
  type response_allCards_edges_node = {
    __typename: string,
    tcgId: string,
    category: [
      | `BATTLE
      | `CHARACTER
      | `STRATAGEM
      | `FutureAddedValue(string)
    ],
    number: string,
    rarity: [
      | `COMMON
      | `PROMO
      | `RARE
      | `SUPER_RARE
      | `THEME
      | `UNCOMMON
      | `FutureAddedValue(string)
    ],
    wave: response_allCards_edges_node_wave,
    id: option(string),
    getFragmentRefs:
      unit =>
      {
        .
        "__$fragment_ref__CardCategory_card": CardCategory_card_graphql.t,
        "__$fragment_ref__BattleCard_battleCard": BattleCard_battleCard_graphql.t,
      },
  };
  type response_allCards_edges = {node: response_allCards_edges_node};
  type response_allCards = {
    edges: option(array(option(response_allCards_edges))),
  };

  type response = {allCards: response_allCards};
  type variables = unit;
};

module Internal = {
  type responseRaw;
  let responseConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"allCards_edges":{"n":"","na":""},"allCards_edges_node_category":{"e":"enum_CardCategory"},"allCards_edges_node_rarity":{"e":"enum_CardRarity"},"allCards_edges_node_wave":{"f":""},"allCards_edges_node_id":{"n":""},"allCards_edges_node":{"f":""}}} |json}
  ];
  let responseConverterMap = {
    "enum_CardCategory": unwrap_enum_CardCategory,
    "enum_CardRarity": unwrap_enum_CardRarity,
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
  "name": "tcgId",
  "args": null,
  "storageKey": null
},
v2 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "category",
  "args": null,
  "storageKey": null
},
v3 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "number",
  "args": null,
  "storageKey": null
},
v4 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "rarity",
  "args": null,
  "storageKey": null
},
v5 = {
  "kind": "ScalarField",
  "alias": null,
  "name": "id",
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
        "concreteType": "CardConnection",
        "plural": false,
        "selections": [
          {
            "kind": "LinkedField",
            "alias": null,
            "name": "edges",
            "storageKey": null,
            "args": null,
            "concreteType": "CardEdge",
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
                  (v2/*: any*/),
                  (v3/*: any*/),
                  (v4/*: any*/),
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
                        "kind": "FragmentSpread",
                        "name": "Wave_wave",
                        "args": null
                      }
                    ]
                  },
                  {
                    "kind": "FragmentSpread",
                    "name": "CardCategory_card",
                    "args": null
                  },
                  {
                    "kind": "InlineFragment",
                    "type": "BattleCard",
                    "selections": [
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
                      (v5/*: any*/)
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
        "concreteType": "CardConnection",
        "plural": false,
        "selections": [
          {
            "kind": "LinkedField",
            "alias": null,
            "name": "edges",
            "storageKey": null,
            "args": null,
            "concreteType": "CardEdge",
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
                  (v2/*: any*/),
                  (v3/*: any*/),
                  (v4/*: any*/),
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
                      (v1/*: any*/),
                      {
                        "kind": "ScalarField",
                        "alias": null,
                        "name": "released",
                        "args": null,
                        "storageKey": null
                      },
                      (v5/*: any*/)
                    ]
                  },
                  (v5/*: any*/),
                  {
                    "kind": "InlineFragment",
                    "type": "BattleCard",
                    "selections": [
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
    "text": "query AllCardsQuery {\n  allCards {\n    edges {\n      node {\n        __typename\n        tcgId\n        category\n        ...CardCategory_card\n        number\n        rarity\n        wave {\n          ...Wave_wave\n          id\n        }\n        ... on BattleCard {\n          ...BattleCard_battleCard\n        }\n        ... on CharacterCard {\n          id\n        }\n        id\n      }\n    }\n  }\n}\n\nfragment BattleCard_battleCard on BattleCard {\n  attackModifier\n  defenseModifier\n  faction\n  icons\n  stars\n  title\n  type_: type\n}\n\nfragment CardCategory_card on Card {\n  category\n}\n\nfragment Wave_wave on Wave {\n  name\n  tcgId\n  released\n}\n",
    "metadata": {}
  }
};
})() |json}
];
