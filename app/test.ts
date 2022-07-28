import {BlockingState, Control, MixedEnum, State, Test} from "./types";

const bs = BlockingState.parse({
  kind: "Allowed",
  reason: "AdClickAttribution"
});

MixedEnum.parse("One")
MixedEnum.parse({Two: "hello...."})
MixedEnum.parse({Three: { temp: 3 }})


const message = Control.parse({ kind: "Start", time: 10 });
if (message.kind === "Start") {
  console.log(message.time) // Type-safe property access
}

// ❌ Invalid enum value. Expected 'Stop' | 'Toggle', received 'oops'
const msg = State.parse({control: "oops"})
