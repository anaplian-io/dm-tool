import { CallToolResult } from '@modelcontextprotocol/sdk/types.js';

export const invokeDmToolApi = async (
  route: string,
): Promise<CallToolResult> => {
  let isError = false;
  const response = await fetch(`http://localhost:8080${route}`)
    .then((response) => response.text())
    .catch((e) => {
      isError = true;
      return String(e);
    });
  return {
    isError,
    content: [
      {
        type: 'text',
        text: response,
      },
    ],
  };
};
