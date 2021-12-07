const rust = require("./native/index.node");

function anotherAsyncCall() {
  return new Promise(resolve => {
    console.log("START*********");
    try {
      const request = {
        url: "someURL",
        method: "GET",
      }
      console.log("RUST*********", rust.run(request));
    } catch (error) {
      console.log("ERROR*********", error);
    }
    console.log("END*********");
  });
}

async function asyncCall() {
  await anotherAsyncCall();
}

asyncCall();