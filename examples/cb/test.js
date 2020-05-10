let addon = require('./dist');
const assert = require('assert');

addon.hello(2,function(msg){
  assert.equal(msg,"argument is: 2");
  console.log("callback test succeed");
});

assert.throws( () => addon.hello(2),{
  message: 'expected argument of type: callback'
}); 

addon.example(function(val){
  assert.equal(val,20);
},10);