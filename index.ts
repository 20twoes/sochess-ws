const server = Bun.serve<{ authToken: string }>({
  fetch(req, server) {
    const success = server.upgrade(req);
    if (success) {
      return undefined;
    }

    return new Response("Hello world!");
  },
  websocket: {
    async message(ws, message) {
      console.log(`Received ${message}`);
      ws.send(`Ack: ${message}`);
    },
  },
});

console.log(`Listening on ${server.hostname}:${server.port}`);
