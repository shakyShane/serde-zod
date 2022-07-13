import z from "zod";


export const TimerResult = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Ended"),
    }),
    z.object({
      kind: z.literal("EndedPrematurely"),
    }),
  ]);

export const Status = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Start"),
      elapsed: z.number(),
      rem: z.number(),
    }),
    z.object({
      kind: z.literal("Tick"),
      elapsed: z.number(),
      rem: z.number(),
    }),
    z.object({
      kind: z.literal("End"),
      result: TimerResult,
    }),
  ]);