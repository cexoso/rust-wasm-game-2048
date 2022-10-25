import { style } from "@vanilla-extract/css";

export const container = style({
  display: "flex",
  flexDirection: "column",
});

export const row = style({
  flexDirection: "row",
  display: "flex",
});

export const cube = style({
  background: "red",
  display: "flex",
  width: 50,
  height: 50,
  margin: 10,
  alignItems: "center",
  justifyContent: "center",
  color: "#fff",
});
