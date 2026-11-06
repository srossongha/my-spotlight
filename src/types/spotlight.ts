export type Action =
  | { type: "LaunchApp"; payload: { path: string } }
  | { type: "CopyToClipboard"; payload: { text: string } }
  | { type: "RunSystemCommand"; payload: { command: string } }
  | { type: "OpenUrl"; payload: { url: string } };

export interface SearchResult {
  id: string;
  title: string;
  subtitle: string;
  icon: string;
  plugin: "apps" | "calculator" | "system" | "web" | string;
  action: Action;
  score: number;
}

