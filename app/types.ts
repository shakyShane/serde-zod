import z from "zod";

export const AllowReason =
  z.enum([
    "ProtectionDisabled",
    "OwnedByFirstParty",
    "RuleException",
    "AdClickAttribution",
    "OtherThirdPartyRequest",
  ])

export const BlockingState =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Blocked"),
    }),
    z.object({
      kind: z.literal("Allowed"),
      reason: AllowReason,
    }),
  ])

export const DetectedRequest =
  z.object({
    url: z.string(),
    state: BlockingState,
    owner_name: z.string().optional(),
    entity_name: z.string().optional(),
    category: z.string().optional(),
    prevalence: z.number().optional(),
    page_url: z.string(),
  })

export const Control =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Stop"),
    }),
    z.object({
      kind: z.literal("Toggle"),
    }),
  ])

export const Test =
  z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("One"),
    }),
    z.object({
      kind: z.literal("Two"),
    }),
  ])

export const TimerResult =
  z.discriminatedUnion("kind", [
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
  ])

export const Status =
  z.discriminatedUnion("kind", [
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
  ])

export const Count2 =
  z.union([
    z.literal("One"),
    z.object({
      Two: z.string(),
    }),
    z.object({
      Three: z.object({
        temp: z.number(),
      }),
    }),
  ])