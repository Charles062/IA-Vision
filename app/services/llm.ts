
export interface Action {
  action: 'click' | 'type';
  x?: number;
  y?: number;
  text?: string;
  description?: string;
}

export interface UIElement {
  control_type: string;
  name: string;
  bounding_box?: [number, number, number, number];
  [key: string]: unknown;
}

export async function generatePlan(userRequest: string, uiTree: UIElement[]): Promise<Action[]> {
  // Filter the UI tree to reduce token count (crucial for local LLMs)
  // We only keep essential fields
  const simplifiedTree = uiTree.slice(0, 50).map(el => {
    // Backend returns bounding_box: [x, y, width, height]
    const [x, y, width, height] = el.bounding_box || [0, 0, 0, 0];
    return {
      type: el.control_type,
      name: el.name,
      x: Math.round(x + width / 2), // Center point
      y: Math.round(y + height / 2)
    };
  }); // Limit items for safety

  const systemPrompt = `You are a Windows Automation Agent.
Your goal is to map a user request to a sequence of actions on the active window.

Tools:
- click(x, y)
- type(text)

Instructions:
1. Analyze the Active Window Elements.
2. Return a JSON Array of actions to fulfill the request.
3. If no actions are needed or possible, return [].
4. Output specific JSON format only. No text.

Format:
[{"action": "click", "x": 100, "y": 200, "description": "Label"}]

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
        model: 'qwen2.5-coder:1.5b', // Using available model
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

    // Cleanup if LLM adds markdown
    content = content.replace(/```json/g, '').replace(/```/g, '').trim();

    // Aggressive JSON extraction: Find the first '[' and the last ']'
    const firstBracket = content.indexOf('[');
    const lastBracket = content.lastIndexOf(']');

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let parsed: any;

    if (firstBracket !== -1 && lastBracket !== -1 && lastBracket > firstBracket) {
      const potentialJson = content.substring(firstBracket, lastBracket + 1);
      try {
        parsed = JSON.parse(potentialJson);
      } catch {
        console.error("Failed to parse extracted JSON:", potentialJson);
        // Fallback to full parse attempt
        try { parsed = JSON.parse(content); } catch { }
      }
    } else {
      // No brackets found, try parsing full content
      try { parsed = JSON.parse(content); } catch { }
    }

    // Handle wrapped responses if we still ended up with an object
    if (!Array.isArray(parsed) && parsed && typeof parsed === 'object') {
      if (parsed.actions && Array.isArray(parsed.actions)) {
        parsed = parsed.actions;
      } else if (parsed.plan && Array.isArray(parsed.plan)) {
        parsed = parsed.plan;
      } else {
        // Basic heuristic: check values for an array
        const values = Object.values(parsed);
        const foundArray = values.find(v => Array.isArray(v));
        if (foundArray) parsed = foundArray;
      }
    }

    // Handle wrapped responses if we still ended up with an object
    if (!Array.isArray(parsed) && parsed && typeof parsed === 'object') {
      if (parsed.actions && Array.isArray(parsed.actions)) {
        parsed = parsed.actions;
      } else if (parsed.plan && Array.isArray(parsed.plan)) {
        parsed = parsed.plan;
      } else if (Object.keys(parsed).length === 0) {
        return [];
      } else {
        // Check if it's a single action object (has 'action' key)
        if (parsed.action) {
          parsed = [parsed];
        } else {
          // Try to find an array in values
          const values = Object.values(parsed);
          const foundArray = values.find(v => Array.isArray(v));
          if (foundArray) parsed = foundArray;
        }
      }
    }

    if (!Array.isArray(parsed)) {
      console.error("LLM Error: Response is not an array (Final fallback)", parsed);
      return [];
    }
    return parsed;
  } catch (e) {
    console.error("LLM Failed:", e);
    // Fallback/Mock for demo if Ollama isn't running
    if (userRequest.toLowerCase().includes("notepad")) {
      return [
        { action: 'type', text: 'Hello from Mock Mode', description: 'Mock typing' }
      ];
    }
    return []; // Failure safe fallback
  }
}
