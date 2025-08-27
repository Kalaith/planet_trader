// Correct type declaration for tool_research.json
declare module '../mocks/tool_research.json' {
  export interface ToolResearch {
    name: string;
    category: string;
    rpCost: number;
    unlocksTool: boolean;
    description: string;
  }
  const value: ToolResearch[];
  export default value;
}
