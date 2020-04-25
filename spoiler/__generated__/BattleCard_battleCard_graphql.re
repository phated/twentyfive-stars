/* @generated */

type enum_BattleType = [
  | `ACTION
  | `SECRET_ACTION
  | `UPGRADE_ARMOR
  | `UPGRADE_UTILITY
  | `UPGRADE_WEAPON
  | `FutureAddedValue(string)
];

let unwrap_enum_BattleType: string => enum_BattleType =
  fun
  | "ACTION" => `ACTION
  | "SECRET_ACTION" => `SECRET_ACTION
  | "UPGRADE_ARMOR" => `UPGRADE_ARMOR
  | "UPGRADE_UTILITY" => `UPGRADE_UTILITY
  | "UPGRADE_WEAPON" => `UPGRADE_WEAPON
  | v => `FutureAddedValue(v);

let wrap_enum_BattleType: enum_BattleType => string =
  fun
  | `ACTION => "ACTION"
  | `SECRET_ACTION => "SECRET_ACTION"
  | `UPGRADE_ARMOR => "UPGRADE_ARMOR"
  | `UPGRADE_UTILITY => "UPGRADE_UTILITY"
  | `UPGRADE_WEAPON => "UPGRADE_WEAPON"
  | `FutureAddedValue(v) => v;

type enum_BattleIcon = [
  | `BLACK
  | `BLUE
  | `GREEN
  | `ORANGE
  | `WHITE
  | `FutureAddedValue(string)
];

let unwrap_enum_BattleIcon: string => enum_BattleIcon =
  fun
  | "BLACK" => `BLACK
  | "BLUE" => `BLUE
  | "GREEN" => `GREEN
  | "ORANGE" => `ORANGE
  | "WHITE" => `WHITE
  | v => `FutureAddedValue(v);

let wrap_enum_BattleIcon: enum_BattleIcon => string =
  fun
  | `BLACK => "BLACK"
  | `BLUE => "BLUE"
  | `GREEN => "GREEN"
  | `ORANGE => "ORANGE"
  | `WHITE => "WHITE"
  | `FutureAddedValue(v) => v;

type enum_Faction = [
  | `AUTOBOT
  | `DECEPTICON
  | `MERCENARY
  | `FutureAddedValue(string)
];

let unwrap_enum_Faction: string => enum_Faction =
  fun
  | "AUTOBOT" => `AUTOBOT
  | "DECEPTICON" => `DECEPTICON
  | "MERCENARY" => `MERCENARY
  | v => `FutureAddedValue(v);

let wrap_enum_Faction: enum_Faction => string =
  fun
  | `AUTOBOT => "AUTOBOT"
  | `DECEPTICON => "DECEPTICON"
  | `MERCENARY => "MERCENARY"
  | `FutureAddedValue(v) => v;

module Types = {
  type fragment = {
    attackModifier: option(int),
    defenseModifier: option(int),
    faction:
      option(
        [ | `AUTOBOT | `DECEPTICON | `MERCENARY | `FutureAddedValue(string)],
      ),
    icons:
      array(
        [
          | `BLACK
          | `BLUE
          | `GREEN
          | `ORANGE
          | `WHITE
          | `FutureAddedValue(string)
        ],
      ),
    stars: option(int),
    title: string,
    type_: [
      | `ACTION
      | `SECRET_ACTION
      | `UPGRADE_ARMOR
      | `UPGRADE_UTILITY
      | `UPGRADE_WEAPON
      | `FutureAddedValue(string)
    ],
  };
};

module Internal = {
  type fragmentRaw;
  let fragmentConverter: Js.Dict.t(Js.Dict.t(Js.Dict.t(string))) = [%raw
    {json| {"__root":{"attackModifier":{"n":""},"defenseModifier":{"n":""},"faction":{"n":"","e":"enum_Faction"},"icons":{"e":"enum_BattleIcon"},"stars":{"n":""},"type_":{"e":"enum_BattleType"}}} |json}
  ];
  let fragmentConverterMap = {
    "enum_Faction": unwrap_enum_Faction,
    "enum_BattleIcon": unwrap_enum_BattleIcon,
    "enum_BattleType": unwrap_enum_BattleType,
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
  {.. "__$fragment_ref__BattleCard_battleCard": t} as 'a;
external getFragmentRef: fragmentRefSelector('a) => fragmentRef = "%identity";

module Utils = {};

type operationType = ReasonRelay.fragmentNode;

let node: operationType = [%raw
  {json| {
  "kind": "Fragment",
  "name": "BattleCard_battleCard",
  "type": "BattleCard",
  "metadata": null,
  "argumentDefinitions": [],
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
} |json}
];
