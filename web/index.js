import init, { plotGraph } from "../pkg/plot.js";

(async () => {
  await init();

  console.group(
    "%cPlotting",
    "color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px"
  );
  plotDemo();
  console.groupEnd();
})();

const plotDemo = () => {
  console.group("Plot");
  let graph = plotGraph("Scatter Plot", "#6044fc");
  console.log(graph);
  document.getElementById("container").innerHTML = graph;

  console.groupEnd();
};
