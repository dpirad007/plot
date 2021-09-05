import init, { plotScatter, plotLine } from "../pkg/plot.js";

(async () => {
  await init();

  let file = await fetch("../data/erupt.csv");

  let scatter = await plotScatter(file, "Dion", 800, 400, 50);

  let line = await plotLine("Line Chart", "#35fcf6");

  document.getElementById("container").innerHTML = scatter;
})();
