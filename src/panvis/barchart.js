var d3 = require('d3');

const barchart = d3.select('#bar-chart');


function renderbarchart(data){
  const xScale = d3.scaleLinear()
    .domain([0,d3.max(data,d => d.Infections)])
    .range([0, barchart.attr('width')]);
  const yScale = d3.scaleBand()
    .domain(data.map(d => d.day))
    .range([0, barchart.attr('height')]);

  barchart.selectAll('rect').data(data)
  .enter().append('rect')
    .attr('color', 'red')
    .attr('y', d => yScale(d.day))
    .attr('width', d => xScale(d.Infections))
    .attr('height', yScale.bandwidth());
};

module.exports = {
    render: renderbarchart
  };