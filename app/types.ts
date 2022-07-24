import z from "zod";

export const AllowReason = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("ProtectionDisabled"),
    }),
    z.object({
      kind: z.literal("OwnedByFirstParty"),
    }),
    z.object({
      kind: z.literal("RuleException"),
    }),
    z.object({
      kind: z.literal("AdClickAttribution"),
    }),
    z.object({
      kind: z.literal("OtherThirdPartyRequest"),
    }),
  ]);

export const BlockingState = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Blocked"),
    }),
    z.object({
      kind: z.literal("Allowed"),
      reason: AllowReason,
    }),
  ]);

export const DetectedRequest = z
  z.object({
    url: z.string(),
    state: BlockingState,
    owner_name: z.string().optional(),
    entity_name: z.string().optional(),
    category: z.string().optional(),
    prevalence: z.number().optional(),
    page_url: z.string(),
  })
