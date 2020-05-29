/* @generated */

module Types = {
  type fragment_wave = {
    getFragmentRefs:
      unit => {. "__$fragment_ref__Wave_wave": Wave_wave_graphql.t},
  };

  type fragment = {
    tcgId: string,
    wave: fragment_wave,
    getFragmentRefs:
      unit =>
      {. "__$fragment_ref__CardCategory_card": CardCategory_card_graphql.t},
  };
};

module Internal = {
  type fragmentRaw;
  let fragmentConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"wave":{"f":""},"":{"f":""}}} |json}
  ];
  let fragmentConverterMap = ();
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
type fragmentRefSelector('a) = {.. "__$fragment_ref__Card_card": t} as 'a;
external getFragmentRef: fragmentRefSelector('a) => fragmentRef = "%identity";

module Utils = {};

type operationType = ReasonRelay.fragmentNode;

let node: operationType = [%raw
  {json| {
  "kind": "Fragment",
  "name": "Card_card",
  "type": "Card",
  "metadata": null,
  "argumentDefinitions": [],
  "selections": [
    {
      "kind": "ScalarField",
      "alias": null,
      "name": "tcgId",
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
    }
  ]
} |json}
];
