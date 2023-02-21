import { Component, createResource, For } from "solid-js";
import { GazouilliCard } from "./components/gazouilli/card";
import { Gazouilli } from "./types/Gazouilli";

const fetchGazouillis = async () =>
  await fetch("http://localhost:8000/gazouilli").then((res) => res.json());

const App: Component = () => {
  const [gazouillis] = createResource<Gazouilli[]>(fetchGazouillis);
  return (
    <div class="p-6 max-w-lg mx-auto">
      <For each={gazouillis()}>{(e) => <GazouilliCard {...e} />}</For>
    </div>
  );
};

export default App;
