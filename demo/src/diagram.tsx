import React from "react";
import ReactFlow from "reactflow";
import "reactflow/dist/style.css";

const FlowComponent = () => {
  const nodes = [
    { id: "1", position: { x: 100, y: 100 }, data: { label: "Node 1" } },
  ];

  const proOptions = { hideAttribution: true };

  return (
    <div style={{ width: "100%", height: "300px", border: "1px solid #000" }}>
      <ReactFlow nodes={nodes} proOptions={proOptions} />
    </div>
  );
};

export default FlowComponent;
