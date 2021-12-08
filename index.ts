const rust = require("./native/index.node");
import fetch from 'node-fetch';

const LOOP_COUNT = 10;
const URL = "YOUR_URL";

export const startExecution: any = async (): Promise<void> => {
  let now = new Date();
  console.log("START native", now);
  const native_promise = [];
  for (let index = 0; index < LOOP_COUNT; index++) {
    native_promise.push(native_fetch());
  }
  await Promise.all(native_promise);
  let diffInMs = new Date().getTime() - now.getTime();
  console.log(`END*********${diffInMs}ms`);

  now = new Date();
  console.log("START node-fetch", now);
  const node_promise = [];
  for (let index = 0; index < LOOP_COUNT; index++) {
    node_promise.push(node_fetch());
  }
  await Promise.all(node_promise);
  diffInMs = new Date().getTime() - now.getTime();
  console.log(`END*********${diffInMs}ms`);
};

async function native_fetch() {
  const request = {
    url: URL,
    method: "GET",
  }
  const response = rust.get(request);
  console.log(JSON.parse(response.data));
}

async function node_fetch() {
  const response = await fetch(URL);
  const body = await response.json();

  console.log(body);
}

(async () => {
  await startExecution();
})();