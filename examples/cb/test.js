let addon = require('./dist');
const assert = require('assert');

addon.hello(2,function(msg){
  assert.equal(msg,"argument is: 2");
  console.log("callback test succeed");
});

assert.throws( () => addon.hello(2),{
  message: '2 args expected but 1 is present'
}); 