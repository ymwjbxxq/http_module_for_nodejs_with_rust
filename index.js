const rust = require("./native/index.node");

(async () => {
  console.log("START*********");
  try {
    const request = {
      url: "https://httpbin.org/ip",
      method: "GET",
    }
    const response = rust.get(request);
    console.log(JSON.parse(response.data));
  } catch (error) {
    console.log(error);
  }
  console.log("END*********");
})();