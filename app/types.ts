import z from "zod";

export const Gender = z
  .discriminatedUnion("kind", [
    z.object({
      kind: z.literal("Male")
    }),
    z.object({
      kind: z.literal("Female")
    }),
  ]);