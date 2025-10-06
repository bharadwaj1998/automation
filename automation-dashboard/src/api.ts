import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:3000",
});

// 🧩 Workflow type — supports both JSON + Postgres formats
export interface Workflow {
  id?: number;
  name?: string;
  nodes?: {
    type: string;
    config: {
      url: string;
      headers?: Record<string, string>;
    };
  }[];
  data?: {
    nodes?: {
      type: string;
      config: {
        url: string;
        headers?: Record<string, string>;
      };
    }[];
  };
}

// 🧠 Run log structure for Live Runs tab
export interface RunLog {
  workflow_id: number;
  timestamp: string;
  status: string;
  result?: any;
}

export const getWorkflows = () => api.get("/workflows");
export const saveWorkflow = (data: Workflow) => api.post("/workflow", data);
export const runWorkflow = (id: number) => api.post(`/run/${id}`);
export const getRuns = () => api.get("/runs");
