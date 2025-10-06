import { useEffect, useState } from "react";
import {
  getWorkflows,
  getRuns,
  runWorkflow,
  saveWorkflow,
  type Workflow,
  type RunLog,
} from "./api";
import WorkflowBuilder from "./components/workflowbuilder";
import { PlayIcon } from "@heroicons/react/24/outline";

export default function App() {
  const [workflows, setWorkflows] = useState<Workflow[]>([]);
  const [runs, setRuns] = useState<RunLog[]>([]);
  const [showBuilder, setShowBuilder] = useState(false);

  // ✅ Load workflows + runs on start
  useEffect(() => {
    getWorkflows().then((res) => setWorkflows(res.data.workflows || []));
    getRuns().then((res) => setRuns(res.data.runs || []));
  }, []);

  // ✅ Connect to live server-sent events
  useEffect(() => {
    const sse = new EventSource("http://localhost:3000/events");
    sse.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        setRuns(data);
      } catch (err) {
        console.error("Error parsing SSE data:", err);
      }
    };
    return () => sse.close();
  }, []);

  // ✅ Save workflow from builder
  const handleSave = async (workflow: any) => {
    await saveWorkflow(workflow);
    const wf = await getWorkflows();
    setWorkflows(wf.data.workflows);
    setShowBuilder(false);
  };

  const handleRun = async (id: number) => {
    await runWorkflow(id);
  };

  return (
    <div className="p-6 max-w-6xl mx-auto font-sans text-gray-900">
      <h1 className="text-3xl font-bold mb-6">⚙️ Automation Builder</h1>

      {!showBuilder ? (
        <>
          <div className="flex justify-between mb-6">
            <h2 className="text-2xl font-semibold">🧩 Workflows</h2>
            <button
              onClick={() => setShowBuilder(true)}
              className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
            >
              ➕ New Workflow
            </button>
          </div>

          {/* Workflows list */}
          <div className="bg-white rounded shadow p-4 mb-8">
            {workflows.length === 0 ? (
              <div className="text-gray-500">No workflows yet.</div>
            ) : (
              workflows.map((wf, i) => (
                <div
                  key={i}
                  className="flex justify-between items-center py-2 border-b last:border-0"
                >
                  <div>
                    Workflow {i + 1}:{" "}
                    {wf.data?.nodes
                      ?.map((n: any) => n.data?.label || "Unnamed Node")
                      .join(" → ")}
                  </div>
                  <button
                    onClick={() => handleRun(wf.id || i)}
                    className="flex items-center gap-1 bg-green-600 text-white px-3 py-1 rounded hover:bg-green-700"
                  >
                    <PlayIcon className="h-5 w-5" /> Run
                  </button>
                </div>
              ))
            )}
          </div>

          {/* Live Runs */}
          <h2 className="text-2xl font-semibold mb-2">📊 Live Runs</h2>
          <div className="bg-gray-900 text-green-400 font-mono p-3 h-64 overflow-y-auto rounded border border-gray-700">
            {runs.length === 0 ? (
              <div className="text-gray-400">Waiting for runs...</div>
            ) : (
              runs.map((r, i) => (
                <div key={i} className="py-1">
                  [{r.timestamp}] Workflow {r.workflow_id} →{" "}
                  {r.status.toUpperCase()}
                </div>
              ))
            )}
          </div>
        </>
      ) : (
        <WorkflowBuilder onSave={handleSave} />
      )}
    </div>
  );
}
