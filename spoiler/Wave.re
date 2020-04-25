module WaveFragment = [%relay.fragment
  {|
  fragment Wave_wave on Wave {
    name
    tcgId
    released
  }
|}
];

[@react.component]
let make = (~wave as waveRef) => {
  let wave = WaveFragment.use(waveRef);
  <div>
    <div> {React.string(wave.name ++ " (" ++ wave.tcgId ++ ")")} </div>
    <div> {React.string("Released on: " ++ wave.released)} </div>
  </div>;
};
