const assert = require('assert');
let addon = require('./dist');


addon.hello(5).then((val) => {
  assert.equal(val,15);
  console.log("promise test succeed: %s",val);
});

(
async () => {
  let val = await addon.hello(5);
  assert.equal(val,15);
})();


(async () => {
  await assert.rejects(
    async () => {
      let val = await addon.hello2(-5);
    },
    {
      message: 'arg is negative'
    }
  );
})();