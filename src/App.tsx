import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

await listen<Payload>('input_update', (event) => {
  let input = event.payload;
  console.log("Payload: " + input.message);
  setUpdate(input.message);
  greet(input.message);
})

class Payload {
  message: string;

  public constructor(message: string) {
    this.message = message;
  }
}

const [update, setUpdate] = createSignal("");
const [greetMsg, setGreetMsg] = createSignal("");
const [name, setName] = createSignal("");

async function greet(message: string) {
  setGreetMsg(await invoke("on_command", { message }));
}

function App() {
  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet("Hello");
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{update()}</p>
    </div>
  );
}

export default App;
