/* @generated */

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
  type fragment = {
    category: [
      | `BATTLE
      | `CHARACTER
      | `STRATAGEM
      | `FutureAddedValue(string)
    ],
  };
};

module Internal = {
  type fragmentRaw;
  let fragmentConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"category":{"e":"enum_CardCategory"}}} |json}
  ];
  let fragmentConverterMap = {"enum_CardCategory": unwrap_enum_CardCategory};
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
  {.. "__$fragment_ref__CardCategory_card": t} as 'a;
external getFragmentRef: fragmentRefSelector('a) => fragmentRef = "%identity";

module Utils = {};

type operationType = ReasonRelay.fragmentNode;

let node: operationType = [%raw
  {json| {
  "kind": "Fragment",
  "name": "CardCategory_card",
  "type": "Card",
  "metadata": null,
  "argumentDefinitions": [],
  "selections": [
    {
      "kind": "ScalarField",
      "alias": null,
      "name": "category",
      "args": null,
      "storageKey": null
    }
  ]
} |json}
];
