// References
// - https://github.com/facundofarias/awesome-websockets

const initialFEN = 'aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq';
const CHANNEL = "game-0";
let fenCache;

const server = Bun.serve<{ authToken: string }>({
  fetch(req, server) {
    // TODO: Use Content Security Policy directives to secure connection
    // https://content-security-policy.com/
    const success = server.upgrade(req);
    if (success) {
      return undefined;
    }

    return new Response("Hello world!");
  },
  websocket: {
    open(ws) {
      ws.subscribe(CHANNEL);
      ws.send(fenCache || initialFEN);
      console.log(`fenCache: ${fenCache}`);
    },
    message(ws, message) {
      console.log(`Received ${message}`);
      //ws.send(`Ack: ${message}`);
      fenCache = message;
      console.log(`fenCache: ${fenCache}`);
      ws.publish(CHANNEL, message);
    },
    close(ws) {
      ws.unsubscribe(CHANNEL);
    },
  },
});

console.log(`Listening on ${server.hostname}:${server.port}`);
