import type { InventoryReport } from "../types";

export function Dashboard({ report }: { report: InventoryReport }) {
  return (
    <section>
      <h2>Dashboard</h2>
      <div>
        {report.tools.map((tool) => (
          <article key={tool.id}>
            <h3>{tool.name}</h3>
            <p>Status: {tool.status}</p>
            <p>Version: {tool.version ?? "-"}</p>
            <p>Warnings: {tool.warnings.length}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
