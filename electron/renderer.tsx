import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource/outfit/400.css";
import "@fontsource/outfit/500.css";
import "@fontsource/outfit/600.css";
import "@fontsource/fraunces/500.css";
import "@fontsource/fraunces/600.css";
import "@fontsource/fraunces/700.css";
import "@/styles.css";
import { GameApp } from "@/components/game/App";

const root = document.getElementById("app");
if (!root) throw new Error("Missing #app");

createRoot(root).render(
  <StrictMode>
    <GameApp />
  </StrictMode>,
);
