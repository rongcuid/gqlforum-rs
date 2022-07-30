import { render } from "preact";
import { App } from "./app";
import "./index.css";

import { createClient, Provider } from "urql";

const client = createClient({
  url: "http://localhost:3000/graphql",
});

render(
  <Provider value={client}>
    <App />
  </Provider>,
  document.getElementById("app") as HTMLElement
);
