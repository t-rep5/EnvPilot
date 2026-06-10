import { useEffect, useState } from "react";
import { scanInventory } from "./api";
import type { InventoryReport } from "./types";
import { Dashboard } from "./pages/Dashboard";
import { Warnings } from "./pages/Warnings";

export function App() {
  const [report, setReport] = useState<InventoryReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    scanInventory().then(setReport).catch((err) => setError(String(err)));
  }, []);

  if (error) {
    return (
      <main>
        <h1>EnvPilot</h1>
        <p>{error}</p>
      </main>
    );
  }

  if (!report) {
    return (
      <main>
        <h1>EnvPilot</h1>
        <p>Scanning...</p>
      </main>
    );
  }

  return (
    <main>
      <h1>EnvPilot</h1>
      <Dashboard report={report} />
      <Warnings warnings={report.warnings} />
    </main>
  );
}
