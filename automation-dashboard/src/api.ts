import axios from "axios";

const API = axios.create({
  baseURL: "http://localhost:3000",
});

export interface Workflow {
  nodes: { type: string; config: Record<string, any> }[];
}

export interface RunLog {
  workflow_id: number;
  timestamp: string;
  status: string;
  result: any;
}

export const getWorkflows = () => API.get<{ workflows: Workflow[] }>("/workflows");
export const getRuns = () => API.get<{ runs: RunLog[] }>("/runs");
export const runWorkflow = (id: number) => API.post(`/run/${id}`);
export const saveWorkflow = (data: Workflow) => API.post("/workflow", data);
