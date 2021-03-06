import React, { Component } from 'react';

import { SampleTrackGraph } from '../api/synthesize';

import createPlotlyComponent from 'react-plotly.js/factory';
import Plotly from "./custom-plotly";
const Plot = createPlotlyComponent(Plotly);

class Graph extends Component {

  render() {
    return (
      <Plot
        className="Graph"
        useResizeHandler
        config={{
          displayModeBar: false
        }}
        data={SampleTrackGraph(this.props.data, this.props.multiplier)}
        layout={{
          showlegend: false,
          autosize: true,
          xaxis: { title: "Time (s)" },
          yaxis: {
            title: "Frequency (Hz)<br />&nbsp;",
            range: [0, this.props.upperRange]
          },
          margin: {
            t: 40,
            l: 85,
            r: 50,
            b: 80,
            pad: 5
          }
        }}
      />
    );
  }

}

export default Graph;
