let addon = require('./dist');
const assert = require('assert');


addon.hello(1,function(val,msg){
  assert.equal(val,10);
  assert.equal(msg,"hello world");
  console.log("callback test succeed");
});


addon.basic(10,function(val,val2){
  assert.equal(val,10);
  assert.equal(val2,20);
  console.log("callback test succeed");
});

