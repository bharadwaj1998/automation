import { useEffect, useState } from "react";
import {
  getWorkflows,
  getRuns,
  runWorkflow,
  saveWorkflow,
  type Workflow,
  type RunLog,
} from "./api";
import { PlayIcon } from "@heroicons/react/24/outline";

export default function App() {
  const [workflows, setWorkflows] = useState<Workflow[]>([]);
  const [runs, setRuns] = useState<RunLog[]>([]);
  const [url, setUrl] = useState("");

  // ✅ Load initial data
  useEffect(() => {
    getWorkflows()
      .then((res) => setWorkflows(res.data.workflows || []))
      .catch((err) => console.error("Failed to fetch workflows:", err));

    getRuns()
      .then((res) => setRuns(res.data.runs || []))
      .catch((err) => console.error("Failed to fetch runs:", err));
  }, []);

  // ✅ Connect to SSE for live updates
  useEffect(() => {
    const sse = new EventSource("http://localhost:3000/events");

    sse.onmessage = (e) => {
      try {
        const data = JSON.parse(e.data);
        console.log("🔥 Live SSE update:", data);
        setRuns(data);
      } catch (err) {
        console.error("Failed to parse SSE data:", err, e.data);
      }
    };

    sse.onerror = (err) => {
      console.error("❌ SSE connection error:", err);
      sse.close();
    };

    return () => sse.close();
  }, []);

  // ✅ Auto-scroll live runs
  useEffect(() => {
    const container = document.getElementById("runs-container");
    if (container) container.scrollTop = container.scrollHeight;
  }, [runs]);

  // ✅ Run workflow
  const handleRun = async (id: number) => {
    console.log(`▶️ Running workflow ${id}`);
    try {
      await runWorkflow(id);
    } catch (err) {
      console.error("Failed to run workflow:", err);
    }
  };

  // ✅ Add workflow
  const handleAddWorkflow = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!url.trim()) return;

    try {
      await saveWorkflow({
        nodes: [
          {
            type: "http",
            config: {
              url,
              headers: { "User-Agent": "automation-dashboard/1.0" },
            },
          },
        ],
      });

      const wf = await getWorkflows();
      setWorkflows(wf.data.workflows);
      setUrl("");
    } catch (err) {
      console.error("Failed to save workflow:", err);
    }
  };

  return (
    <div className="p-6 max-w-5xl mx-auto font-sans text-gray-900">
      <h1 className="text-3xl font-bold mb-6">⚙️ Automation Dashboard (Real-time)</h1>

      {/* ➕ Add Workflow */}
      <form onSubmit={handleAddWorkflow} className="flex gap-2 mb-6">
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Enter API URL (e.g. https://api.github.com)"
          className="flex-grow border border-gray-300 rounded px-3 py-2"
        />
        <button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
          Add
        </button>
      </form>

      {/* 🧩 Workflows Section */}
      <h2 className="text-2xl font-semibold mb-2">🧩 Workflows</h2>
      <div className="bg-white rounded shadow p-4 mb-8">
        {workflows.length === 0 ? (
          <div className="text-gray-500">No workflows added yet.</div>
        ) : (
          workflows.map((wf, i) => {
            // ✅ Handle both Postgres + JSON structures
            const url =
              wf?.nodes?.[0]?.config?.url ||
              wf?.data?.nodes?.[0]?.config?.url ||
              "(no URL)";

            return (
              <div
                key={wf.id ?? i}
                className="flex justify-between items-center py-2 border-b last:border-0"
              >
                <div>
                  <span className="font-mono text-sm text-gray-600">
                    Workflow {wf.id ?? i}
                  </span>
                  : {url}
                </div>
                <button
                  onClick={() => handleRun(wf.id ?? i)}
                  className="flex items-center gap-1 bg-green-600 text-white px-3 py-1 rounded hover:bg-green-700"
                >
                  <PlayIcon className="h-5 w-5" />
                  Run
                </button>
              </div>
            );
          })
        )}
      </div>

      {/* 📊 Live Runs Section */}
      <h2 className="text-2xl font-semibold mb-2">📊 Live Runs</h2>
      <div
        id="runs-container"
        className="bg-gray-900 text-green-400 font-mono p-3 h-64 overflow-y-auto rounded border border-gray-700"
      >
        {runs.length === 0 ? (
          <div className="text-gray-400">Waiting for runs...</div>
        ) : (
          runs.map((r, i) => (
            <div key={i} className="py-1">
              <span className="text-gray-500">[{r.timestamp}]</span>{" "}
              <span>Workflow {r.workflow_id} → </span>
              <span
                className={
                  r.status === "success" ? "text-green-400" : "text-red-400"
                }
              >
                {r.status?.toUpperCase?.() ?? "UNKNOWN"}
              </span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
