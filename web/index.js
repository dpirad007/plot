import init, { plotGraph } from "../pkg/plot.js";

(async () => {
  await init();
  let graph = plotGraph("Line chart", "#35fcf6");
  console.log(graph);
  document.getElementById("container").innerHTML = graph;
})();
