var d3 = require('d3');

const linechart = d3.select('#line-chart');
const width = +linechart.attr('width');
const height = +linechart.attr('height');

function renderlinechart(data){
    const title = 'Virus Development';
    const xAxisLabel = 'Time';
    const yAxisLabel = 'count';
    // set the margins
    const margin = { top: 60, right: 40, bottom: 88, left: 105 };
    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;
    
    // set the ranges
    const x = d3.scaleLinear().range([0, innerWidth]);

    const y = d3.scaleLinear().range([innerHeight, 0]);
    
    const xAxis = d3.axisBottom(x)
        .tickSize(-innerHeight)
        .tickPadding(15);
  
    const yAxis = d3.axisLeft(y)
        .tickSize(-innerWidth)
        .tickPadding(10);

    // define the 1st line
    const valueline = d3.line()
        .x(function(d) { return x(d.day); })
        .y(function(d) { return y(d.Infections); });

    const valueline2 = d3.line()
        .x(function(d) { return x(d.day);})
        .y(function(d) { return y(d.Deaths);});
    
    const valueline3 = d3.line()
        .x(function(d) { return x(d.day);})
        .y(function(d) { return y(d.Recoveries);});
        
    const valueline4 = d3.line()
        .x(function(d) { return x(d.day);})
        .y(function(d) { return y(d.Active);});

    const valueline5 = d3.line()
        .x(function(d) { return x(d.day);})
        .y(function(d) { return y(d.NewInfections);});
        
    const g = linechart.append('g')
        .attr('transform', `translate(${margin.left},${margin.top})`); 

    x.domain(d3.extent(data, function(d) { return d.day; }));
    y.domain([0, d3.max(data, function(d) {
        return Math.max(d.Infections, d.Deaths, d.Recoveries, d.Active, d.NewInfections); })]);
        
    // Add the valueline path.
    g.append("path")
        .data([data])
        .style("stroke", "blue")
        .attr('class', 'line')
        .attr("d", valueline);

    // Add the valueline2 path.
    g.append("path")
        .data([data])
        .style("stroke", "red")
        .attr('class', 'line')
        .attr("d", valueline2);

    g.append("path")
        .data([data])
        .style("stroke", "green")
        .attr('class', 'line')
        .attr("d", valueline3);

    g.append("path")
        .data([data])
        .style("stroke", "black")
        .attr('class', 'line')
        .attr("d", valueline4);

    g.append("path")
        .data([data])
        .style("stroke", "magenta")
        .attr('class', 'line')
        .attr("d", valueline5);

    // Add the X Axis
    const yAxisG = g.append('g').call(yAxis);
    yAxisG.selectAll('.domain').remove();
    
    yAxisG.append('text')
        .attr('class', 'axis-label')
        .attr('y', -80)
        .attr('x', -innerHeight / 2)
        .attr('fill', 'black')
        .attr('transform', `rotate(-90)`)
        .attr('text-anchor', 'middle')
        .text(yAxisLabel);
    
    const xValue = d => d.day;
    const xScale = d3.scaleLinear()
        .domain(d3.extent(data, xValue))
        .range([0, innerWidth])
        .nice();

    const xAxisG = g.append('g').call(xAxis)
      .attr('transform', `translate(0,${innerHeight})`);
    
    xAxisG.select('.domain').remove();
    
    xAxisG.append('text')
        .attr('class', 'axis-label')
        .attr('y', +80)
        .attr('x', innerWidth / 2)
        .attr('fill', 'black')
        .text(xAxisLabel);

    g.append('text')
        .attr('class', 'title')
        .attr('y', 0)
        .text(title);
};

  
 

  
module.exports = {
    render: renderlinechart
  };