import z from "zod";
import {BlockingState} from "./types";

const bs = BlockingState.parse({
  kind: "Allowed",
  reason: "AdClickAttribution"
});

export const b1 = z.union([
  z.object({
    One: z.string(),
  }),
  z.literal("Two")
])

b1.parse("Two");
