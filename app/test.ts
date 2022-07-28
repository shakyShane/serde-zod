import {Count2, BlockingState} from "./types";

const bs = BlockingState.parse({
  kind: "Allowed",
  reason: "AdClickAttribution"
});

Count2.parse("One")
Count2.parse({Two: "hello...."})
Count2.parse({Three: { temp: 3 }})
