const d3 = require('d3');
const barchart = require('./barchart.js');
const linechart = require('./linechart.js');



d3.csv('./data.csv').then(data => {
  for (let index = 0; index < data.length; index++) {
      
      data[index].Infections = +data[index].Infections;
      data[index].Deaths = +data[index].Deaths;
      data[index].Recoveries = +data[index].Recoveries;
      data[index].Active = +data[index].Active;
      data[index].NewInfections = +data[index].NewInfections;
      data[index].day = index;
      }
  //console.log(data);
  //barchart.render(data);
  linechart.render(data);
});

function simulate(){

};
