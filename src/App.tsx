import { h, render } from "preact";
import { signal } from "@preact/signals";
import { Suspense, lazy } from "preact/compat";
import init from "./wasm/pkg/advent_of_code.js";

const Day01 = lazy(() => import("./components/Day01.js"));
const Day02 = lazy(() => import("./components/Day02.js"));
const Day03 = lazy(() => import("./components/Day03.js"));
const Day04 = lazy(() => import("./components/Day04.js"));
const Day05 = lazy(() => import("./components/Day05.js"));
const Day06 = lazy(() => import("./components/Day06.js"));
const Day07 = lazy(() => import("./components/Day07.js"));
const Day08 = lazy(() => import("./components/Day08.js"));

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
          <Suspense fallback="">
            <Day02 />
          </Suspense>
          <Suspense fallback="">
            <Day03 />
          </Suspense>
          <Suspense fallback="">
            <Day04 />
          </Suspense>
          <Suspense fallback="">
            <Day05 />
          </Suspense>
          <Suspense fallback="">
            <Day06 />
          </Suspense>
          <Suspense fallback="">
            <Day07 />
          </Suspense>
          <Suspense fallback="">
            <Day08 />
          </Suspense>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
        </div>
      )}
    </div>
  );
}

render(<App />, document.getElementById("app") as HTMLDivElement);
