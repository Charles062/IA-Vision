
export interface Action {
  action: 'click' | 'type';
  x?: number;
  y?: number;
  text?: string;
  description?: string;
}

export async function generatePlan(userRequest: string, uiTree: any[]): Promise<Action[]> {
  // Filter the UI tree to reduce token count (crucial for local LLMs)
  // We only keep essential fields
  const simplifiedTree = uiTree.map(el => ({
    type: el.control_type,
    name: el.name,
    x: el.x + el.width / 2, // Center point
    y: el.y + el.height / 2
  })).slice(0, 50); // Limit items for safety

  const systemPrompt = `You are a Windows Automation Agent.
Your goal is to map a user request to a sequence of actions on the active window.
You are provided with a list of UI Elements currently visible.

Tools available:
- click(x, y): Click at coordinates.
- type(text): Type text at current location (usually after a click).

Return ONLY a valid JSON array of objects with keys: "action" ("click" or "type"), "x", "y", "text" (for type), "description".
Do not include markdown blocks.

Example:
[
  {"action": "click", "x": 100, "y": 200, "description": "Click File menu"},
  {"action": "type", "text": "Hello", "description": "Type Hello"}
]
`;

  const userPrompt = `Request: "${userRequest}"
Active Window Elements:
${JSON.stringify(simplifiedTree)}
`;

  console.log("Sending to Ollama...");

  try {
      const response = await fetch('http://localhost:11434/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: 'llama3', // or 'mistral'
          messages: [
            { role: 'system', content: systemPrompt },
            { role: 'user', content: userPrompt }
          ],
          stream: false,
          format: 'json' // Enforce JSON mode if supported
        })
      });

      if (!response.ok) {
        throw new Error(`Ollama API Error: ${response.statusText}`);
      }

      const data = await response.json();
      let content = data.message.content;
      console.log("Raw LLM Response:", content);

      // Cleanup if LLM adds markdown
      content = content.replace(/```json/g, '').replace(/```/g, '').trim();

      return JSON.parse(content);
  } catch (e) {
      console.error("LLM Failed:", e);
      // Fallback/Mock for demo if Ollama isn't running
      if (userRequest.toLowerCase().includes("notepad")) {
          return [
              { action: 'type', text: 'Hello from Mock Mode', description: 'Mock typing' }
          ];
      }
      throw e;
  }
}
