import z from "zod";
import parse from "date-fns/parseISO";
// import {BlockingState, Control, MixedEnum, State, Test} from "./types";
//
// const bs = BlockingState.parse({
//   kind: "Allowed",
//   reason: "AdClickAttribution"
// });
//
// MixedEnum.parse("One")
// MixedEnum.parse({Two: "hello...."})
// MixedEnum.parse({Three: { temp: 3 }})
//
//
// const message = Control.parse({ kind: "Start", time: 10 });
// if (message.kind === "Start") {
//   console.log(message.time) // Type-safe property access
// }
//
// // ❌ Invalid enum value. Expected 'Stop' | 'Toggle', received 'oops'
// const msg = State.parse({control: "oops"})
// import z from "zod";
//
// const Names = z.array(z.number())
//
// Names.parse([1_000]);

// let input_2 = "2022-08-02T21:30:14+01:00";
// let input_3 = "2022-08-02T21:30:14Z";
// let input_4 = "Tue, 02 Aug 2022 21:36:08 +0100";
// let input_5 = "2022-08-02T21:30:14BST";
//
// var d = new Date("2022-08-02T21:57:20.672227BST"); /* midnight in China on April 13th */
// let s = d.toLocaleString('en-GB', { timeZone: 'Europe/London' });
//
// console.log(s)
// const dateSchema = z.preprocess((arg) => new Date(arg), z.date());
// const dateSchema = z.preprocess((arg) => new Date(arg), z.date());
//
// let v2 = dateSchema.parse(input_2);
// console.log({v2});
//
// let v3 = dateSchema.parse(input_3);
// console.log({v3});
//
// let v4 = dateSchema.parse(input_4);
// console.log({v4});
//
// let s = new Set([1, 2]);
// let sz = z.preprocess((incoming) => new Set([...incoming]), z.set(z.number()));
// let parsed = sz.parse(["1", "2"]);
// console.log(parsed);

let obs = z.record(z.number());
let incoming = { b: 3, a: 3 }
obs.parse(incoming)
