import z from "zod";


export const Test = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("One"),
    }),
    z.object({
      kind: z.literal("Two"),
    }),
  ]);

export const Control = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Stop"),
    }),
    z.object({
      kind: z.literal("Toggle"),
    }),
  ]);

export const TimerResult = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Ended"),
    }),
    z.object({
      kind: z.literal("EndedPrematurely"),
      after: z.number(),
    }),
    z.object({
      kind: z.literal("Other"),
      items: z.array(z.array(Test)),
    }),
    z.object({
      kind: z.literal("WithOptional"),
      control: Control.optional(),
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