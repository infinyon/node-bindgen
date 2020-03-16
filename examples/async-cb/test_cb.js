let addon = require('./dist');
const assert = require('assert');

addon.hello(1,function(val,msg){
  assert.equal(val,10);
  assert.equal(msg,"hello world");
  console.log("callback test succeed");
});

