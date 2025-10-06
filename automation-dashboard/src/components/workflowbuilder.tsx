import React, { useCallback } from "react";
import ReactFlow, {
  addEdge,
  useNodesState,
  useEdgesState,
  MiniMap,
  Controls,
  Background,
  type Connection,
  type Edge,
  type Node,
} from "reactflow";
import "reactflow/dist/style.css";

interface WorkflowBuilderProps {
  onSave: (workflow: any) => Promise<void>;
}

export default function WorkflowBuilder({ onSave }: WorkflowBuilderProps) {
  const [nodes, setNodes, onNodesChange] = useNodesState([
    {
      id: "1",
      type: "default",
      position: { x: 250, y: 5 },
      data: { label: "🌐 HTTP Request Node" },
    },
  ]);

  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  // ✅ Add new connection between nodes
  const onConnect = useCallback(
    (params: Edge | Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  // ✅ Add new node
  const addNewNode = useCallback(() => {
    setNodes((nds) => {
      const id = (nds.length + 1).toString();
      const newNode: Node = {
        id,
        type: "default",
        position: { x: 150 + nds.length * 50, y: 100 + nds.length * 50 },
        data: { label: `🧩 Node ${id}` },
      };
      return [...nds, newNode];
    });
  }, [setNodes]);

  // ✅ Save workflow
  const handleSaveClick = async () => {
    const workflow = { nodes, edges };

    try {
      const response = await fetch("http://localhost:3000/workflow", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(workflow),
      });

      if (!response.ok) throw new Error("Failed to save workflow");
      const data = await response.json();
      alert("✅ Workflow saved successfully!");
      onSave(data);
    } catch (err) {
      console.error("❌ Error saving workflow:", err);
      alert("Failed to save workflow. Check console.");
    }
  };

  return (
    <div className="h-screen w-full bg-gray-50">
      <div className="p-3 bg-white border-b shadow-sm flex justify-between items-center">
        <h2 className="text-lg font-semibold">🧩 Workflow Builder</h2>
        <div className="flex gap-3">
          <button
            onClick={addNewNode}
            className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
          >
            ➕ Add Node
          </button>
          <button
            onClick={handleSaveClick}
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
      >
        <MiniMap />
        <Controls />
        <Background gap={12} size={1} color="#ddd" />
      </ReactFlow>
    </div>
  );
}
