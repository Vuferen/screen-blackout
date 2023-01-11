import "./app.postcss";
import "./style.css";
import App from "./Blackout.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
