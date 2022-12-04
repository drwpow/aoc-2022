import { h, render } from "preact";
import { signal } from "@preact/signals";
import { Suspense, lazy } from "preact/compat";
import init from "./wasm/pkg/advent_of_code.js";

const Day01 = lazy(() => import("./components/Day01.js"));

const status = signal<"READY" | "LOADING" | "SUCCESS">("READY");

function App() {
  if (status.value === "READY") {
    status.value = "LOADING";
    init().then(() => {
      status.value = "SUCCESS";
    });
  }

  return (
    <div class="wrapper">
      <h1>Advent of Code 2022</h1>
      {status.value === "SUCCESS" && (
        <div class="cal">
          <div class="cal-heading">S</div>
          <div class="cal-heading">M</div>
          <div class="cal-heading">T</div>
          <div class="cal-heading">W</div>
          <div class="cal-heading">T</div>
          <div class="cal-heading">F</div>
          <div class="cal-heading">S</div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <Suspense fallback="">
            <Day01 />
          </Suspense>
          <div class="day"></div>
          <div class="day"></div>
        </div>
      )}
    </div>
  );
}

render(<App />, document.getElementById("app") as HTMLDivElement);
