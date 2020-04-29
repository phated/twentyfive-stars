module AppContainer = {
  open Emotion;

  let container = [%css
    [
      display(grid),
      gridTemplateColumns(
        list([repeat(autoFill, [minmax(px(200), fr(1.0))])]),
      ),
      gridGap(rem(1.0)),
      padding(rem(0.5)),
    ]
  ];

  [@react.component]
  let make = () => {
    <ReasonRelay.Context.Provider environment=RelayEnv.environment>
      <ReactExperimental.Suspense fallback=React.null>
        <div className=container> <AllCards key="cards" /> </div>
      </ReactExperimental.Suspense>
    </ReasonRelay.Context.Provider>;
  };
};

ReactExperimental.renderConcurrentRootAtElementWithId(
  <AppContainer />,
  "app",
);
