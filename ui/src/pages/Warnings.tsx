import type { Diagnostic } from "../types";

export function Warnings({ warnings }: { warnings: Diagnostic[] }) {
  return (
    <section>
      <h2>Warnings</h2>
      {warnings.length === 0 ? (
        <p>No global warnings.</p>
      ) : (
        <ul>
          {warnings.map((warning, index) => (
            <li key={`${warning.code}-${index}`}>
              {warning.severity}: {warning.message}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
