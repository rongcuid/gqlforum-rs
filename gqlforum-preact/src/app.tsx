import { useState } from "preact/hooks";
import preactLogo from "./assets/preact.svg";
import "./app.css";
import { useQuery } from "urql";

const query = `
query {
  user(by: {id: 1}) {
    name
  }
}
`;

export function App() {
  const [count, setCount] = useState(0);
  const [result, reexecuteQuery] = useQuery({
    query: query,
  });
  const { data, fetching, error } = result;
  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo" alt="Vite logo" />
        </a>
        <a href="https://preactjs.com" target="_blank">
          <img src={preactLogo} class="logo preact" alt="Preact logo" />
        </a>
      </div>
      <h1>Vite + Preact</h1>
      <div class="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/app.tsx</code> and save to test HMR
        </p>
      </div>
      {fetching ? <p>LOADING</p> : ""}
      {error ? <p>ERROR</p> : ""}
      {data ? <p>{data.user.name}</p> : ""}
      <p class="read-the-docs">
        Click on the Vite and Preact logos to learn more
      </p>
    </>
  );
}
