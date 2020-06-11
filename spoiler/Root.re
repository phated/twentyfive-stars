module RootQuery = [%relay.query
  {|
    query RootQuery {
      ...AllCards_query
    }
  |}
];

[@react.component]
let make = () => {
  let root = RootQuery.use(~variables=(), ());
  <AllCards query={root.getFragmentRefs()} />;
};
