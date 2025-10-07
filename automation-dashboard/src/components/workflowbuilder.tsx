import React, { useCallback, useState } from "react";
import ReactFlow, {
  addEdge,
  useNodesState,
  useEdgesState,
  MiniMap,
  Controls,
  Background,
  Handle,
  Position,
} from "reactflow";
import type { Connection, Edge, Node } from "reactflow";

import "reactflow/dist/style.css";

export default function WorkflowBuilder({ onSave }: { onSave: (workflow: any) => void }) {
  const [nodes, setNodes, onNodesChange] = useNodesState([
    {
      id: "1",
      type: "httpNode",
      position: { x: 200, y: 50 },
      data: { label: "HTTP Request", url: "https://api.github.com" },
    },
  ]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onConnect = useCallback(
  (params: Edge | Connection) =>
    setEdges((eds: Edge[]) => addEdge(params, eds)),
  [setEdges]
);


  const addNewNode = useCallback(() => {
   setNodes((nds: Node[]) => {
  const id = (nds.length + 1).toString();
  const newNode: Node = {
    id,
    type: "httpNode",
    position: { x: 200 + nds.length * 50, y: 150 + nds.length * 50 },
    data: { label: `HTTP Node ${id}`, url: "" },
  };
  return [...nds, newNode];
});

  }, [setNodes]);

  const handleSave = () => {
    const workflow = {
      nodes: nodes.map((n) => ({
        type: "http",
        config: { url: n.data.url || "" },
      })),
      edges,
    };
    onSave(workflow);
  };

  return (
    <div className="h-screen w-full bg-gray-50">
      <div className="p-3 bg-white border-b shadow-sm flex justify-between items-center">
        <h2 className="text-lg font-semibold">🧩 Workflow Builder</h2>
        <div className="flex gap-2">
          <button
            onClick={addNewNode}
            className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
          >
            ➕ Add Node
          </button>
          <button
            onClick={handleSave}
            className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 transition"
          >
            💾 Save Workflow
          </button>
        </div>
      </div>

      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
        style={{ background: "#f8fafc" }}
        nodeTypes={{ httpNode: HttpNode }}
      >
        <MiniMap />
        <Controls />
        <Background gap={12} size={1} color="#ddd" />
      </ReactFlow>
    </div>
  );
}

// ✅ Custom node component with editable URL
function HttpNode({ data }: any) {
  return (
    <div className="bg-white border border-gray-300 rounded-md shadow p-2 w-56">
      <div className="font-bold mb-1 text-sm">🌐 {data.label}</div>
      <input
        type="text"
        value={data.url}
        onChange={(e) => (data.url = e.target.value)}
        placeholder="Enter URL"
        className="border w-full rounded px-2 py-1 text-sm"
      />
      <Handle type="target" position={Position.Top} />
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}
