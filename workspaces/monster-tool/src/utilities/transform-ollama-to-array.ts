import ollama from 'ollama';
import { isArray } from './guards';

type Option<T> =
  | {
      readonly type: 'some';
      readonly some: T;
    }
  | {
      readonly type: 'none';
      readonly error: string;
    };

export interface TransformOllamaToArrayProps<T extends object> {
  readonly rawText: string;
  readonly examples: {
    readonly parsed: T[];
    readonly input: string;
  }[];
  readonly typeGuard: (obj: unknown) => obj is T;
}

export const transformOllamaToArray = async <T extends object>(
  props: TransformOllamaToArrayProps<T>,
): Promise<Option<T[]>> => {
  const { rawText, examples, typeGuard } = props;
  const formattedExamples = examples
    .map(
      (example, index) => `
Example Input ${index}
\`\`\`text
${example.input}
\`\`\`

\`\`\`json
${JSON.stringify(example.parsed)}
\`\`\`
`,
    )
    .join('\n');
  const prompt = `
You are a natural language processing algorithm. You accept a piece of raw text and parse it into a well-formed JSON array consistent with the shape given in the examples (your output must always be an array).
Your output shape must exactly match the shape specified in the examples.

${formattedExamples}

Input
\`\`\`text
${rawText}
\`\`\`
  `;
  const modelResponse = await ollama.chat({
    stream: false,
    model: 'gemma3:latest',
    messages: [
      {
        role: 'user',
        content: prompt,
      },
    ],
  });
  const extractedJsonString = modelResponse.message.content
    .replace(/.*```json(.*?)```.*/s, '$1')
    .replaceAll(' ', '')
    .replaceAll('\n', '')
    .trim();
  let parsedJson: unknown;
  try {
    parsedJson = JSON.parse(extractedJsonString);
  } catch (e) {
    const errorMessage = `Failed to parse JSON from ${modelResponse.message.content}`;
    console.warn(errorMessage, e);
    return {
      type: 'none',
      error: errorMessage,
    };
  }
  if (isArray(parsedJson)) {
    return {
      type: 'some',
      some: parsedJson.filter(typeGuard),
    };
  }
  console.warn('Did not receive an array as a response.');
  return {
    type: 'none',
    error: `Did not receive an array as a response.`,
  };
};
