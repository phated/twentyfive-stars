module AppContainer = {
  [@react.component]
  let make = () => {
    <ReasonRelay.Context.Provider environment=RelayEnv.environment>
      <ReactExperimental.Suspense fallback=React.null>
        <Root key="cards" />
      </ReactExperimental.Suspense>
    </ReasonRelay.Context.Provider>;
  };
};

ReactExperimental.renderConcurrentRootAtElementWithId(
  <AppContainer />,
  "app",
);
