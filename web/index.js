import init, { plotScatter, plotLine } from "../pkg/plot.js";

(async () => {
  await init();

  let scatterfile = await fetch("../data/faithful.csv");

  let linefile = await fetch("../data/line.csv");

  let scatter = await plotScatter(
    scatterfile,
    "Faithful Dataset",
    800,
    400,
    50
  );

  // let line = await plotLine(linefile, "Line Chart");

  document.getElementById("container").innerHTML = scatter;
})();
